use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::column::ledger_drcr::Model,
        journal::transaction::column::ledger_drcr::ActiveModel,
        JournalTransactionColumnId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::column::ledger_drcr::Model,
    ) -> Result<journal::transaction::column::ledger_drcr::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp, template_column_id, amount) 
                    VALUES($1, $2, $3, $4) RETURNING *",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.template_column_id,
                    &model.amount,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::column::ledger_drcr::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionColumnId>>,
    ) -> Result<Vec<journal::transaction::column::ledger_drcr::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2 AND template_column_id=$3::TemplateColumnId",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(
                            search_one.as_str(),
                            &[&id.journal_id(), &id.timestamp(), &id.template_column_id()],
                        )
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
        let mut records = Vec::<journal::transaction::column::ledger_drcr::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::column::ledger_drcr::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::column::ledger_drcr::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::column::ledger_drcr::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2 AND template_column_id=$3::TemplateColumnId",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[&id.journal_id(), &id.timestamp(), &id.template_column_id()],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, WHERE journal_id=$1::JournalId AND timestamp=$2 AND template_column_id=$3::TemplateColumnId",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[&id.journal_id(), &id.timestamp(), &id.template_column_id()],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE journal_id=$1::JournalId AND timestamp=$2 AND template_column_id=$3::TemplateColumnId",
            journal::transaction::column::ledger_drcr::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[&id.journal_id(), &id.timestamp(), &id.template_column_id()],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::column::ledger_drcr::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            template_column_id: value.get("template_column_id"),
            amount: value.get("amount"),
            ledger_cr_id: value.get("ledger_cr_id"),
            ledger_dr_id: value.get("ledger_dr_id"),
            column_total_id: value.get("total_column_id"),
        }
    }
}
