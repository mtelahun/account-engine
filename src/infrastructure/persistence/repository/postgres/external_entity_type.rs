use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::external_entity::entity_code::EntityCode,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::external,
};

#[async_trait]
impl
    RepositoryOperations<
        external::entity_type::Model,
        external::entity_type::ActiveModel,
        EntityCode,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &external::entity_type::Model,
    ) -> Result<external::entity_type::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(code, description) VALUES($1, $2) RETURNING *",
            external::entity_type::ActiveModel::NAME
        );
        let res = conn
            .query_one(&query, &[&model.code, &model.description])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(external::entity_type::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<EntityCode>>,
    ) -> Result<Vec<external::entity_type::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            external::entity_type::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", external::entity_type::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<external::entity_type::ActiveModel>::new();
        for row in rows {
            let am = external::entity_type::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::entity_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::entity_type::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET description = $1 WHERE code = $2",
            external::entity_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&model.description, &model.code])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: EntityCode) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1",
            external::entity_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: EntityCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: EntityCode) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for external::entity_type::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            code: value.get("code"),
            description: value.get("description"),
        }
    }
}
