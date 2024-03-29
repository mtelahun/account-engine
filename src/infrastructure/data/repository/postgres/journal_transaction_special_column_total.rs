use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::special_journal::column_total_id::ColumnTotalId,
    infrastructure::data::db_context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
    shared_kernel::{JournalTransactionId, Sequence},
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::column::sum::Model,
        journal::transaction::special::column::sum::ActiveModel,
        ColumnTotalId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::column::sum::Model,
    ) -> Result<journal::transaction::special::column::sum::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(id, journal_id, timestamp, total, posting_ref_cr, posting_ref_dr, sequence) 
                    VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &ColumnTotalId::new(),
                    &model.summary_id.journal_id(),
                    &model.summary_id.timestamp(),
                    &model.amount,
                    &model.posting_ref_cr,
                    &model.posting_ref_dr,
                    &model.sequence,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::column::sum::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<ColumnTotalId>>,
    ) -> Result<Vec<journal::transaction::special::column::sum::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::ColumntTotalId[])",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<journal::transaction::special::column::sum::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::column::sum::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::column::sum::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &journal::transaction::special::column::sum::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET journal_id = $1, timestamp = $2, sequence = $3, total = $4, posting_ref_cr = $5, posting_ref_dr = $6 WHERE id = $7::ColumnTotalId;",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.summary_id.journal_id(),
                &model.summary_id.timestamp(),
                &model.sequence,
                &model.amount,
                &model.posting_ref_cr,
                &model.posting_ref_dr,
                &model.id,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: ColumnTotalId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::ColumnTotalId;",
            journal::transaction::special::column::sum::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, _id: ColumnTotalId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: ColumnTotalId) -> Result<u64, OrmError> {
        todo!()
    }
}

impl From<Row> for journal::transaction::special::column::sum::ActiveModel {
    fn from(value: Row) -> Self {
        let sequence: i16 = value.get("sequence");
        let sequence = Sequence::try_from(sequence).unwrap();
        let tx_id = JournalTransactionId::new(value.get("journal_id"), value.get("timestamp"));
        Self {
            id: value.get("id"),
            summary_id: tx_id,
            sequence,
            amount: value.get("total"),
            posting_ref_cr: value.get("posting_ref_cr"),
            posting_ref_dr: value.get("posting_ref_dr"),
        }
    }
}
