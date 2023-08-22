use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::AccountId,
    resource::external,
    store::{OrmError, Resource, ResourceOperations},
};

use super::store::PostgresStore;

#[async_trait]
impl ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &external::account::Model,
    ) -> Result<external::account::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, subsidiary_ledger, entity_code, account_no, opened_date) VALUES($1, $2, $3, $4, $5) RETURNING *",
            external::account::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[
                    &AccountId::new(),
                    &model.subledger_id,
                    &model.entity_type_code,
                    &model.account_no,
                    &model.date_opened,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(external::account::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<external::account::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            external::account::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", external::account::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<external::account::ActiveModel>::new();
        for row in rows {
            let am = external::account::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<external::account::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::account::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET subsidiary_ledger_id = $1, entity_code = $2, account_no = $3, date_opened = $4 WHERE id = $5::AccountId;",
            external::account::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.subledger_id,
                &model.entity_type_code,
                &model.account_no,
                &model.date_opened,
                &model.id,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::AccountId;",
            external::account::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for external::account::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            subledger_id: value.get("subsidiary_ledger_id"),
            entity_type_code: value.get("entity_type_code"),
            account_no: value.get("account_no"),
            date_opened: value.get("opened_date"),
        }
    }
}
