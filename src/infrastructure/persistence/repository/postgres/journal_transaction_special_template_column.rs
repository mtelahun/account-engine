use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::entity::special_journal_template_column::template_column_id::TemplateColumnId,
    infrastructure::persistence::context::{
        error::OrmError, postgres::PostgresStore, repository_operations::RepositoryOperations,
        resource::Resource,
    },
    resource::journal,
    shared_kernel::Sequence,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::template::column::Model,
        journal::transaction::special::template::column::ActiveModel,
        TemplateColumnId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::template::column::Model,
    ) -> Result<journal::transaction::special::template::column::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(id, template_id, name, seq, column_type, dr_ledger_id, cr_ledger_id) 
                    VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            journal::transaction::special::template::column::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &TemplateColumnId::new(),
                    &model.template_id,
                    &model.name,
                    &model.sequence,
                    &model.column_type,
                    &model.dr_ledger_id,
                    &model.cr_ledger_id,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::template::column::ActiveModel::from(res))
    }

    async fn get(
        &self,
        ids: Option<&Vec<TemplateColumnId>>,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::SpecialJournalTemplateColId[])",
            journal::transaction::special::template::column::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::special::template::column::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records =
            Vec::<journal::transaction::special::template::column::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::template::column::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::template::column::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, id: TemplateColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::SpecialJournalTemplateColId;",
            journal::transaction::special::template::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: TemplateColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, WHERE id=$1::SpecialJournalTemplateColId",
            journal::transaction::special::template::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: TemplateColumnId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id=$1::SpecialJournalTemplateColId",
            journal::transaction::special::template::column::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::special::template::column::ActiveModel {
    fn from(value: Row) -> Self {
        let sequence: i16 = value.get("seq");
        let sequence = Sequence::try_from(sequence).unwrap();
        Self {
            id: value.get("id"),
            template_id: value.get("template_id"),
            sequence,
            name: value.get("name"),
            column_type: value.get("column_type"),
            dr_ledger_id: value.get("dr_ledger_id"),
            cr_ledger_id: value.get("cr_ledger_id"),
            dr_account_id: value.get("dr_account_id"),
            cr_account_id: value.get("cr_account_id"),
        }
    }
}
