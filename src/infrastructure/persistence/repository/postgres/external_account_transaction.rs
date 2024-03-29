use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::external_account::account_transaction_id::AccountTransactionId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::external,
};

#[async_trait]
impl
    RepositoryOperations<
        external::account::transaction::Model,
        external::account::transaction::ActiveModel,
        AccountTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &external::account::transaction::Model,
    ) -> Result<external::account::transaction::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(external_account_id, timestamp, xact_type_code, amount) VALUES($1, $2, $3, $4) RETURNING *",
            external::account::transaction::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                &query,
                &[
                    &model.external_account_id,
                    &model.timestamp,
                    &model.xact_type_code,
                    &model.amount,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(external::account::transaction::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountTransactionId>>,
    ) -> Result<Vec<external::account::transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE external_account_id = $1::AccountId AND timestamp = $2",
            external::account::transaction::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            external::account::transaction::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = match ids {
            Some(ids) => {
                let mut temp_rows = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.account_id(), &id.timestamp()])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_rows.append(&mut res);
                    }
                }

                temp_rows
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<external::account::transaction::ActiveModel>::new();
        for row in rows {
            let am = external::account::transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::account::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &external::account::transaction::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET xact_type_code = $1, amount = $2
                WHERE external_account_id = $3::AccountId AND timestamp = $4;",
            external::account::transaction::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.xact_type_code,
                &model.amount,
                &model.external_account_id,
                &model.timestamp,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: AccountTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE external_account_id = $1::AccountId and timestamp = $2;",
            external::account::transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.account_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: AccountTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: AccountTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for external::account::transaction::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            external_account_id: value.get("external_account_id"),
            timestamp: value.get("timestamp"),
            xact_type_code: value.get("xact_type_code"),
            amount: value.get("amount"),
        }
    }
}
