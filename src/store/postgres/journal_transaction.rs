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
        journal::transaction::Model,
        journal::transaction::ActiveModel,
        JournalTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::Model,
    ) -> Result<journal::transaction::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(journal_id, timestamp, explanation) 
                VALUES($1, $2, $3) RETURNING *",
            journal::transaction::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.journal_id, &model.timestamp, &model.explanation],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", journal::transaction::ActiveModel::NAME);
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
        let mut records = Vec::<journal::transaction::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &journal::transaction::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, state = 'archived' WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            explanation: value.get("explanation"),
        }
    }
}
