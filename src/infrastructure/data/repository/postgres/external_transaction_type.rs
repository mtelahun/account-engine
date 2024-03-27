use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::ExternalXactTypeCode,
    infrastructure::data::db_context::postgres::PostgresStore,
    resource::external,
    store::{OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl
    ResourceOperations<
        external::transaction_type::Model,
        external::transaction_type::ActiveModel,
        ExternalXactTypeCode,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &external::transaction_type::Model,
    ) -> Result<external::transaction_type::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(code, description) VALUES($1, $2) RETURNING *",
            external::transaction_type::ActiveModel::NAME
        );
        let res = conn
            .query_one(&query, &[&model.code, &model.description])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(external::transaction_type::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<ExternalXactTypeCode>>,
    ) -> Result<Vec<external::transaction_type::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            external::transaction_type::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            external::transaction_type::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<external::transaction_type::ActiveModel>::new();
        for row in rows {
            let am = external::transaction_type::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::transaction_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::transaction_type::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET description = $1 WHERE code = $2",
            external::transaction_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&model.description, &model.code])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1",
            external::transaction_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for external::transaction_type::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            code: value.get("code"),
            description: value.get("description"),
        }
    }
}
