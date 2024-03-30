use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::journal::journal_id::JournalId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
};

#[async_trait]
impl RepositoryOperations<journal::Model, journal::ActiveModel, JournalId> for PostgresStore {
    async fn insert(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, name, code, journal_type) VALUES($1, $2, $3, $4) RETURNING *",
            journal::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[
                    &JournalId::new(),
                    &model.name,
                    &model.code,
                    &model.journal_type,
                ],
            )
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
            "UPDATE {} SET name = $1, code = $2, WHERE id = $3::JournalId;",
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

impl From<Row> for journal::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            code: value.get("code"),
            journal_type: value.get("journal_type"),
            control_ledger_id: value.get("ledger_id"),
            template_id: value.get("template_id"),
        }
    }
}
