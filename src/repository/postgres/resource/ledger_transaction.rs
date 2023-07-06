use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    entity::{ledger_transaction, LedgerKey},
    orm::{OrmError, Resource, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl ResourceOperations<ledger_transaction::Model, ledger_transaction::ActiveModel, LedgerKey>
    for PostgresRepository
{
    async fn insert(
        &self,
        model: &ledger_transaction::Model,
    ) -> Result<ledger_transaction::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(ledger_id, timestamp, ledger_dr_id)
                VALUES($1, $2, $3) RETURNING *",
            ledger_transaction::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.ledger_id, &model.timestamp, &model.ledger_dr_id],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_transaction::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<ledger_transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id=$1::AccountId AND timestamp=$2",
            ledger_transaction::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_transaction::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.ledger_id, &id.timestamp])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<ledger_transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger_transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<ledger_transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger_transaction::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerKey) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE ledger_id = $1 AND timestamp = $2",
            ledger_transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.ledger_id, &id.timestamp])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE ledger_id = $1 AND timestamp = $2",
            ledger_transaction::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for ledger_transaction::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            ledger_id: value.get("ledger_id"),
            timestamp: value.get("timestamp"),
            ledger_dr_id: value.get("ledger_dr_id"),
        }
    }
}
