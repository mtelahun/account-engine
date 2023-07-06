use std::iter::zip;

use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::{ids::JournalId, JournalTransactionId, XactType},
    entity::{
        journal, journal_transaction, journal_transaction_line_ledger, ledger_line,
        ledger_transaction, LedgerKey, PostingRef, TransactionState,
    },
    orm::{JournalService, OrmError, Resource, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl ResourceOperations<journal::Model, journal::ActiveModel, JournalId> for PostgresRepository {
    async fn insert(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, name, code) VALUES($1, $2, $3) RETURNING *",
            journal::ActiveModel::NAME
        );
        let res = conn
            .query_one(&query, &[&JournalId::new(), &model.name, &model.code])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            journal::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", journal::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<journal::ActiveModel>::new();
        for row in rows {
            let am = journal::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<journal::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &journal::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET name = $1, code = $2 WHERE id = $3::JournalId;",
            journal::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&model.name, &model.code, &model.id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: JournalId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::JournalId;",
            journal::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: JournalId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::JournalId;",
            journal::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::JournalId;",
            journal::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

#[async_trait]
impl JournalService for PostgresRepository {
    async fn create(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        Ok(self.insert(model).await?)
    }

    async fn add_transaction(
        &self,
        _model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        todo!()
    }

    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, OrmError> {
        let ledger_xact_type = self.get_journal_entry_type(id).await?;

        let mut jxact_lines = <Self as ResourceOperations<
            journal_transaction_line_ledger::Model,
            journal_transaction_line_ledger::ActiveModel,
            JournalTransactionId,
        >>::get(self, Some(&vec![id]))
        .await?;
        let cr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        let mut ledger_posted_list = Vec::<journal_transaction_line_ledger::ActiveModel>::new();
        for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
            let key = LedgerKey {
                ledger_id: cr.ledger_id,
                timestamp: cr.timestamp,
            };
            let entry = ledger_line::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: cr.amount,
                journal_ref: id,
            };
            let tx_dr = ledger_transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_dr_id: dr.ledger_id,
            };

            let _ = self.insert(&entry).await?;
            let _ = self.insert(&tx_dr).await?;
            let mut cr = *cr;
            cr.state = TransactionState::Posted;
            cr.posting_ref = Some(PostingRef {
                key,
                account_id: cr.ledger_id,
            });
            let mut dr = *dr;
            dr.state = TransactionState::Posted;
            dr.posting_ref = Some(PostingRef {
                key,
                account_id: dr.ledger_id,
            });
            ledger_posted_list.push(dr);
            ledger_posted_list.push(cr);
        }

        for line in jxact_lines.iter_mut() {
            for post_line in ledger_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.update_journal_transaction_line_ledger_posting_ref(id, post_line)
                        .await?;
                }
            }
        }

        Ok(true)
    }
}

impl From<Row> for journal::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            code: value.get("code"),
        }
    }
}
