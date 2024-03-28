use std::str::FromStr;

use async_trait::async_trait;
use tokio_postgres::Row;

use crate::domain::{ArrayString128, GeneralLedgerId};
use crate::infrastructure::data::db_context::error::OrmError;
use crate::infrastructure::data::db_context::postgres::PostgresStore;
use crate::infrastructure::data::db_context::repository_operations::RepositoryOperations;
use crate::infrastructure::data::db_context::resource::Resource;
use crate::resource::general_ledger;

#[async_trait]
impl RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
    for PostgresStore
{
    async fn insert(
        &self,
        _model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let gl: Vec<general_ledger::ActiveModel> = self.get(None).await?;

        Ok(gl[0])
    }

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

impl From<Row> for general_ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayString128::from_str(value.get("name")).unwrap_or_default(),
            root: value.get("root"),
            currency_code: value.get("currency_code"),
        }
    }
}
