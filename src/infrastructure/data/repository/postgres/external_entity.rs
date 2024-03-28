use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::EntityId,
    infrastructure::data::db_context::{
        error::OrmError, postgres::PostgresStore, repository_operations::ResourceOperations,
        resource::Resource,
    },
    resource::external,
};

#[async_trait]
impl ResourceOperations<external::entity::Model, external::entity::ActiveModel, EntityId>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &external::entity::Model,
    ) -> Result<external::entity::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {} (id, entity_type_code, name) VALUES($1, $2, $3) RETURNING *",
            external::entity::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[&EntityId::new(), &model.entity_type_code, &model.name],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(external::entity::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<EntityId>>,
    ) -> Result<Vec<external::entity::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1::EntityId",
            external::entity::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", external::entity::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<external::entity::ActiveModel>::new();
        for row in rows {
            let am = external::entity::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<external::entity::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::entity::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET entity_type_code = $1, name = $2 WHERE id = $3::EntityId;",
            external::entity::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[&model.entity_type_code, &model.name, &model.id],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: EntityId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::EntityId;",
            external::entity::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: EntityId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: EntityId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for external::entity::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            entity_type_code: value.get("entity_type_code"),
            name: value.get("name"),
        }
    }
}
