#![allow(clippy::diverging_sub_expression)]
use async_trait::async_trait;
use chronoutil::RelativeDuration;
use tokio_postgres::Row;

use crate::{
    domain::period::period_id::PeriodId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::accounting_period,
};

#[async_trait]
impl RepositoryOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, fiscal_year, period_start, period_end) VALUES($1, $2, $3, $4) RETURNING *",
            accounting_period::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[
                    &PeriodId::new(),
                    &model.fiscal_year,
                    &model.period_start,
                    &(model.period_start + RelativeDuration::years(1) + RelativeDuration::days(-1)),
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(accounting_period::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            accounting_period::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", accounting_period::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<accounting_period::ActiveModel>::new();
        for row in rows {
            let am = accounting_period::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _active_model: &accounting_period::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: PeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: PeriodId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::PeriodId;",
            accounting_period::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: PeriodId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::PeriodId;",
            accounting_period::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for accounting_period::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            fiscal_year: value.get("fiscal_year"),
            period_start: value.get("period_start"),
            period_end: value.get("period_end"),
            period_type: value.get("period_type"),
        }
    }
}
