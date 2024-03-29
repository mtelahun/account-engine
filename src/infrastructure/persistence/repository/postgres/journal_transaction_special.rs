use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::{
        general_journal_transaction::journal_transaction_id::JournalTransactionId,
        subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
    },
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::Model,
        journal::transaction::special::ActiveModel,
        JournalTransactionId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::Model,
    ) -> Result<journal::transaction::special::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(journal_id, timestamp, template_id, transaction_type_external_code) 
                VALUES($1, $2, $3, $4) RETURNING *",
            journal::transaction::special::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.template_id,
                    &model.xact_type_external,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", journal::transaction::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.journal_id(), &id.timestamp()])
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
        let mut records = Vec::<journal::transaction::special::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, state = 'archived' WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal::transaction::special::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id.journal_id(), &id.timestamp()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::special::ActiveModel {
    fn from(value: Row) -> Self {
        let xact_type_external_code: String = value.get("transaction_type_external_code");
        let xact_type_external_code = ExternalXactTypeCode::from(xact_type_external_code);
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            xact_type_external: Some(xact_type_external_code),
            template_id: value.get("template_id"),
        }
    }
}
