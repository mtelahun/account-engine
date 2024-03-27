use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    infrastructure::data::db_context::postgres::PostgresStore,
    resource::{ledger::transaction, LedgerKey},
    store::{OrmError, Resource, ResourceOperations},
};

#[async_trait]
impl ResourceOperations<transaction::ledger::Model, transaction::ledger::ActiveModel, LedgerKey>
    for PostgresStore
{
    async fn insert(
        &self,
        model: &transaction::ledger::Model,
    ) -> Result<transaction::ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(ledger_id, timestamp, ledger_dr_id)
                VALUES($1, $2, $3) RETURNING *",
            transaction::ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.ledger_id, &model.timestamp, &model.ledger_dr_id],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(transaction::ledger::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id=$1::LedgerId AND timestamp=$2",
            transaction::ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", transaction::ledger::ActiveModel::NAME);
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
        let mut records = Vec::<transaction::ledger::ActiveModel>::new();
        for row in rows {
            let am = transaction::ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &transaction::ledger::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1",
            transaction::ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE ledger_id = $1 AND timestamp = $2",
            transaction::ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.ledger_id, &id.timestamp])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: LedgerKey) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE ledger_id = $1 AND timestamp = $2",
            transaction::ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for transaction::ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            ledger_id: value.get("ledger_id"),
            timestamp: value.get("timestamp"),
            ledger_dr_id: value.get("ledger_dr_id"),
        }
    }
}
