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
impl RepositoryOperations<ledger::intermediate::Model, ledger::intermediate::ActiveModel, LedgerId>
    for PostgresStore
{
    async fn get(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::intermediate::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::LedgerId[])",
            ledger::intermediate::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::intermediate::ActiveModel>::new();
        for row in rows {
            let am = ledger::intermediate::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<ledger::intermediate::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(
        &self,
        model: &ledger::intermediate::Model,
    ) -> Result<ledger::intermediate::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id) VALUES($1) RETURNING *;",
            ledger::intermediate::ActiveModel::NAME
        );
        let res = conn
            .query_one(query.as_str(), &[&model.id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::intermediate::ActiveModel::from(res))
    }

    async fn save(&self, _model: &ledger::intermediate::ActiveModel) -> Result<u64, OrmError> {
        // let conn = self.get_connection().await?;
        // let query = format!(
        //     "UPDATE {} SET 1 = 1 WHERE id = $1::LedgerId;",
        //     ledger::intermediate::ActiveModel::NAME
        // );

        // conn.execute(
        //     query.as_str(),
        //     &[
        //         &model.id,
        //     ],
        // )
        // .await
        // .map_err(|e| OrmError::Internal(e.to_string()))

        Ok(0)
    }

    async fn delete(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::LedgerId;",
            ledger::intermediate::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::LedgerId;",
            ledger::intermediate::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::LedgerId;",
            ledger::intermediate::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger::intermediate::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
        }
    }
}
