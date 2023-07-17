use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::{ledger_xact_type_code, JournalTransactionId, LedgerXactTypeCode},
    resource::{ledger, LedgerKey},
    store::{postgres::store::PostgresStore, OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &ledger::transaction::Model,
    ) -> Result<ledger::transaction::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(ledger_id, timestamp, ledger_transaction_type_code, amount, journal_id)
                VALUES($1, $2, $3, $4, $5) RETURNING *",
            ledger::transaction::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.ledger_id,
                    &model.timestamp,
                    &model.ledger_xact_type_code,
                    &model.amount,
                    &model.journal_ref.journal_id(),
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::transaction::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id=$1::AccountId AND timestamp=$2",
            ledger::transaction::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::transaction::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.ledger_id, &id.timestamp])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<ledger::transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger::transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::transaction::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerKey) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE ledger_id = $1 AND timestamp = $2",
            ledger::transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.ledger_id, &id.timestamp])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE ledger_id = $1 AND timestamp = $2",
            ledger::transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.ledger_id, &id.timestamp])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger::transaction::ActiveModel {
    fn from(value: Row) -> Self {
        let journal_ref =
            JournalTransactionId::new(value.get("journal_id"), value.get("timestamp"));
        let mut lxtc: String = value.get("ledger_transaction_type_code");
        if lxtc.len() > ledger_xact_type_code::LEN {
            lxtc.truncate(ledger_xact_type_code::LEN);
        }
        let lxtc = LedgerXactTypeCode::from(lxtc);
        Self {
            ledger_id: value.get("ledger_id"),
            timestamp: value.get("timestamp"),
            ledger_xact_type_code: lxtc,
            amount: value.get("amount"),
            journal_ref,
        }
    }
}
