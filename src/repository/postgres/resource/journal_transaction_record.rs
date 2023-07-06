use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::JournalTransactionId,
    entity::journal_transaction_record,
    orm::{OrmError, Resource, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl
    ResourceOperations<
        journal_transaction_record::Model,
        journal_transaction_record::ActiveModel,
        JournalTransactionId,
    > for PostgresRepository
{
    async fn insert(
        &self,
        model: &journal_transaction_record::Model,
    ) -> Result<journal_transaction_record::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(journal_id, timestamp, explanation) 
                VALUES($1, $2, $3) RETURNING *",
            journal_transaction_record::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.journal_id, &model.timestamp, &model.explanation],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal_transaction_record::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction_record::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_record::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal_transaction_record::ActiveModel::NAME
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
        let mut records = Vec::<journal_transaction_record::ActiveModel>::new();
        for row in rows {
            let am = journal_transaction_record::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal_transaction_record::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal_transaction_record::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, state = 'archived' WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_record::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_record::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal_transaction_record::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            explanation: value.get("explanation"),
        }
    }
}
