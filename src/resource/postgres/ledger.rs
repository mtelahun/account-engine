use std::str::FromStr;

use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::{AccountId, ArrayLongString, ArrayShortString},
    entity::ledger,
    resource::{postgres::repository::PostgresRepository, OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId> for PostgresRepository {
    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::AccountId[])",
            ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::ActiveModel>::new();
        for row in rows {
            let am = ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, ledger_no, name, ledger_type, parent_id, currency_code) VALUES($1, $2, $3, $4, $5, $6) RETURNING *;",
            ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                query.as_str(),
                &[
                    &AccountId::new(),
                    &model.ledger_no.as_str(),
                    &model.name.as_str(),
                    &model.ledger_type,
                    &model.parent_id,
                    &model.currency_code,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::ActiveModel::from(res))
    }

    async fn save(&self, model: &ledger::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET name = $1, currency_code = $2, parent_id = $3, ledger_type = $4, ledger_no = $5 WHERE id = $6::LedgerId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.name,
                &model.currency_code,
                &model.parent_id,
                &model.ledger_type,
                &model.ledger_no,
                &model.id,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::LedgerId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::AccountId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::AccountId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayLongString::from_str(value.get("name")).unwrap_or_default(),
            ledger_no: ArrayShortString::from_str(value.get("ledger_no")).unwrap_or_default(),
            ledger_type: value.get("ledger_type"),
            parent_id: value.get("parent_id"),
            currency_code: value.get("currency_code"),
        }
    }
}

impl Resource for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl Resource for ledger::derived::ActiveModel {
    const NAME: &'static str = "ledger_derived";
}
