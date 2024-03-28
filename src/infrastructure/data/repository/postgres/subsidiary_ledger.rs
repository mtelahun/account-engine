use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::SubLedgerId,
    infrastructure::data::db_context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::subsidiary_ledger,
};

#[async_trait]
impl RepositoryOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &subsidiary_ledger::Model,
    ) -> Result<subsidiary_ledger::ActiveModel, OrmError> {
        let id = SubLedgerId::new();
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, name, ledger_id) VALUES($1, $2, $3) RETURNING *",
            subsidiary_ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(&query, &[&id, &model.name, &model.ledger_id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(subsidiary_ledger::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<SubLedgerId>>,
    ) -> Result<Vec<subsidiary_ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1:SubLedgerId",
            subsidiary_ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", subsidiary_ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<subsidiary_ledger::ActiveModel>::new();
        for row in rows {
            let am = subsidiary_ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<subsidiary_ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &subsidiary_ledger::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET name = $1, ledger_id = $2 WHERE id = $3:SubLedgerId",
            subsidiary_ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&model.name, &model.ledger_id, &model.id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: SubLedgerId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1:SubLedgerId",
            subsidiary_ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: SubLedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: SubLedgerId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for subsidiary_ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            ledger_id: value.get("ledger_id"),
        }
    }
}
