use std::str::FromStr;

use async_trait::async_trait;
use tokio_postgres::Row;

use crate::domain::{ArrayLongString, GeneralLedgerId};
use crate::entity::{accounting_period, general_ledger, journal, ledger};
use crate::orm::{GeneralLedgerService, OrmError, Resource, ResourceOperations};
use crate::repository::postgres::repository::PostgresRepository;

#[async_trait]
impl ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
    for PostgresRepository
{
    async fn get(
        &self,
        ids: Option<&Vec<GeneralLedgerId>>,
    ) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::GeneralLedgerId[]);",
            general_ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {};", general_ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<general_ledger::ActiveModel>::new();
        for row in rows {
            let am = general_ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(
        &self,
        _model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let gl: Vec<general_ledger::ActiveModel> = self.get(None).await?;

        Ok(gl[0])
    }

    async fn save(&self, model: &general_ledger::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET name = $1, currency_code = $2 WHERE id = $3::GeneralLedgerId;",
            general_ledger::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[&model.name, &model.currency_code, &model.id],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, _id: GeneralLedgerId) -> Result<u64, OrmError> {
        Ok(0)
    }

    async fn archive(&self, id: GeneralLedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::GeneralLedgerId;",
            general_ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: GeneralLedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::GeneralLedgerId;",
            general_ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

#[async_trait]
impl GeneralLedgerService for PostgresRepository {
    async fn init(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let root: Vec<ledger::ActiveModel> = self.get(None).await?;
        let gl: Vec<general_ledger::ActiveModel> = self.get(None).await?;
        let mut gl = gl[0];
        gl.name = model.name;
        gl.currency_code = model.currency_code;
        gl.root = root[0].id;
        let _ = self.save(&gl).await?;

        Ok(gl)
    }

    async fn add_journal(&self, _model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        todo!()
    }

    async fn add_ledger(&self, _model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        todo!()
    }

    async fn add_period(
        &self,
        _model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        todo!()
    }
}

impl From<Row> for general_ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayLongString::from_str(value.get("name")).unwrap_or_default(),
            root: value.get("root"),
            currency_code: value.get("currency_code"),
        }
    }
}
