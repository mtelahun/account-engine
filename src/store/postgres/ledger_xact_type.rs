use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::ledger_xact_type_code::{self, LedgerXactTypeCode},
    resource::ledger_xact_type,
    store::{postgres::store::PostgresStore, OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl ResourceOperations<ledger_xact_type::Model, ledger_xact_type::ActiveModel, LedgerXactTypeCode>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &ledger_xact_type::Model,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(code, description) VALUES($1, $2) RETURNING *;",
            ledger_xact_type::ActiveModel::NAME
        );
        let res = conn
            .query_one(query.as_str(), &[&model.code, &model.description])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_xact_type::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerXactTypeCode>>,
    ) -> Result<Vec<ledger_xact_type::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE code = any ($1)",
            ledger_xact_type::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_xact_type::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger_xact_type::ActiveModel>::new();
        for row in rows {
            let am = ledger_xact_type::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger_xact_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger_xact_type::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE code = $1",
            ledger_xact_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE code = $1",
            ledger_xact_type::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger_xact_type::ActiveModel {
    fn from(value: Row) -> Self {
        let mut code: String = value.get("code");
        if code.len() > ledger_xact_type_code::LEN {
            code.truncate(ledger_xact_type_code::LEN);
        }
        let code = LedgerXactTypeCode::from(code);

        Self {
            code,
            description: value.get("description"),
        }
    }
}
