use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::JournalTransactionId,
    resource::journal,
    store::{postgres::store::PostgresStore, OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::line::account::Model,
        journal::transaction::line::account::ActiveModel,
        JournalTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::line::account::Model,
    ) -> Result<journal::transaction::line::account::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp, account_id, xact_type, amount, state) 
                    VALUES($1, $2, $3, $4, $5, $6) RETURNING *",
            journal::transaction::line::account::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.account_id,
                    &model.xact_type,
                    &model.amount,
                    &model.state,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::line::account::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::line::account::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::line::account::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::line::account::ActiveModel::NAME
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
        let mut records = Vec::<journal::transaction::line::account::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::line::account::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::line::account::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::line::account::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::line::account::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::line::account::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::line::account::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            account_id: value.get("account_id"),
            xact_type: value.get("xact_type"),
            amount: value.get("amount"),
            state: value.get("state"),
            posting_ref: value.get("posting_ref"),
        }
    }
}