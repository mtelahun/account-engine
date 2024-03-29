use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::general_journal_transaction::journal_transaction_id::JournalTransactionId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
    shared_kernel::Sequence,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::column::Model,
        journal::transaction::special::column::ActiveModel,
        JournalTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::column::Model,
    ) -> Result<journal::transaction::special::column::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp, dr_ledger_id, cr_ledger_id, amount, state, posting_ref) 
                    VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            journal::transaction::special::column::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.dr_ledger_id,
                    &model.cr_ledger_id,
                    &model.amount,
                    &model.state,
                    &model.column_total_id,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::column::ActiveModel::from(
            res,
        ))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::column::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::special::column::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.journal_id(), &id.timestamp()])
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
        let mut records = Vec::<journal::transaction::special::column::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::column::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::column::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::special::column::ActiveModel {
    fn from(value: Row) -> Self {
        let sequence: i16 = value.get("sequence");
        let sequence = Sequence::try_from(sequence).unwrap();
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            sequence,
            dr_ledger_id: value.get("dr_ledger_id"),
            cr_ledger_id: value.get("cr_ledger_id"),
            amount: value.get("amount"),
            state: value.get("state"),
            column_total_id: value.get("column_total_id"),
        }
    }
}
