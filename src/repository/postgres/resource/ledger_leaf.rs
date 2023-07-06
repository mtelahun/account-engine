use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::AccountId,
    entity::{ledger, ledger_leaf},
    orm::{OrmError, Resource, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl ResourceOperations<ledger_leaf::Model, ledger_leaf::ActiveModel, AccountId>
    for PostgresRepository
{
    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger_leaf::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::AccountId)",
            ledger_leaf::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger_leaf::ActiveModel>::new();
        for row in rows {
            let am: ledger_leaf::ActiveModel = ledger_leaf::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger_leaf::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(
        &self,
        model: &ledger_leaf::Model,
    ) -> Result<ledger_leaf::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id) VALUES($1) RETURNING *;",
            ledger_leaf::ActiveModel::NAME
        );
        let res = conn
            .query_one(query.as_str(), &[&model.id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_leaf::ActiveModel::from(res))
    }

    async fn save(&self, _model: &ledger_leaf::ActiveModel) -> Result<u64, OrmError> {
        // let conn = self.get_connection().await?;
        // let query = format!(
        //     "UPDATE {} SET 1 = 1 WHERE id = $1::LedgerId;",
        //     ledger_leaf::ActiveModel::NAME
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

    async fn delete(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::LedgerId;",
            ledger_leaf::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::AccountId;",
            ledger_leaf::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::AccountId;",
            ledger_leaf::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger_leaf::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
        }
    }
}
