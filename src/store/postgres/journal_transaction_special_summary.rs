use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::JournalTransactionId,
    resource::journal,
    store::{OrmError, Resource, ResourceOperations},
};

use super::store::PostgresStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::summary::Model,
        journal::transaction::special::summary::ActiveModel,
        JournalTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::summary::Model,
    ) -> Result<journal::transaction::special::summary::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp) 
                    VALUES($1, $2) RETURNING *",
            journal::transaction::special::summary::ActiveModel::NAME
        );
        let res = conn
            .query_one(sql.as_str(), &[&model.journal_id, &model.timestamp])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::summary::ActiveModel::from(
            res,
        ))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::summary::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id = $1::JournalId AND timestamp = $2)",
            journal::transaction::special::summary::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::special::summary::ActiveModel::NAME
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
        let mut records = Vec::<journal::transaction::special::summary::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::summary::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::summary::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &journal::transaction::special::summary::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET journal_id = $1, timestamp = $2 WHERE journal_id = $3::JournalId AND timestamp = $4",
            journal::transaction::special::summary::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.journal_id,
                &model.timestamp,
                &model.journal_id,
                &model.timestamp,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE journal_id = $1::JournalId AND timestamp = $2;",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for journal::transaction::special::summary::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
        }
    }
}
