use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::ledger,
    shared_kernel::LedgerId,
};

#[async_trait]
impl RepositoryOperations<ledger::derived::Model, ledger::derived::ActiveModel, LedgerId>
    for PostgresStore
{
    async fn get(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::derived::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::LedgerId[])",
            ledger::derived::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::derived::ActiveModel>::new();
        for row in rows {
            let am = ledger::derived::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::derived::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(
        &self,
        model: &ledger::derived::Model,
    ) -> Result<ledger::derived::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {} (id) VALUES($1) RETURNING *;",
            ledger::derived::ActiveModel::NAME
        );
        let res = conn
            .query_one(query.as_str(), &[&model.id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::derived::ActiveModel::from(res))
    }

    async fn save(&self, _model: &ledger::derived::ActiveModel) -> Result<u64, OrmError> {
        // let conn = self.get_connection().await?;
        // let query = format!(
        //     "UPDATE {} SET 1 = 1 WHERE id = $1::LedgerId;",
        //     ledger::derived::ActiveModel::NAME
        // );

        // conn.execute(
        //     query.as_str(),
        //     &[
        //         &model.id,
        //     ],
        // )
        // .await
        // .map_err(|e| OrmError::Internal(e.to_string()));

        Ok(0)
    }

    async fn delete(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::LedgerId;",
            ledger::derived::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::LedgerId;",
            ledger::derived::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::LedgerId;",
            ledger::derived::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger::derived::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
        }
    }
}
