use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::ids::InterimPeriodId,
    resource::accounting_period::interim_period,
    store::{postgres::store::PostgresStore, OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl ResourceOperations<interim_period::Model, interim_period::ActiveModel, InterimPeriodId>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &interim_period::Model,
    ) -> Result<interim_period::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, parent_id, interim_start, interim_end) VALUES($1, $2, $3, $4) RETURNING *",
            interim_period::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[
                    &InterimPeriodId::new(),
                    &model.parent_id,
                    &model.start,
                    &model.end,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(interim_period::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_period::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            interim_period::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", interim_period::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<interim_period::ActiveModel>::new();
        for row in rows {
            let am = interim_period::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<interim_period::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &interim_period::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: InterimPeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: InterimPeriodId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::InterimPeriodId;",
            interim_period::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: InterimPeriodId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::InterimPeriodId;",
            interim_period::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for interim_period::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            parent_id: value.get("parent_id"),
            start: value.get("interim_start"),
            end: value.get("interim_end"),
        }
    }
}
