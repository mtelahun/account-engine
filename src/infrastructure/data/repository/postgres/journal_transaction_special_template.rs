use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::SpecialJournalTemplateId,
    infrastructure::data::db_context::{
        error::OrmError, postgres::PostgresStore, repository_operations::ResourceOperations,
        resource::Resource,
    },
    resource::journal,
};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::template::Model,
        journal::transaction::special::template::ActiveModel,
        SpecialJournalTemplateId,
    > for PostgresStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::template::Model,
    ) -> Result<journal::transaction::special::template::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(id, name) 
                    VALUES($1, $2) RETURNING *",
            journal::transaction::special::template::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&SpecialJournalTemplateId::new(), &model.name],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::transaction::special::template::ActiveModel::from(
            res,
        ))
    }

    async fn get(
        &self,
        ids: Option<&Vec<SpecialJournalTemplateId>>,
    ) -> Result<Vec<journal::transaction::special::template::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::SpecialJournalTemplateId[])",
            journal::transaction::special::template::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal::transaction::special::template::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<journal::transaction::special::template::ActiveModel>::new();
        for row in rows {
            let am = journal::transaction::special::template::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::template::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::template::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::SpecialJournalTemplateId;",
            journal::transaction::special::template::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true, WHERE id=$1::SpecialJournalTemplateId",
            journal::transaction::special::template::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id=$1::SpecialJournalTemplateId",
            journal::transaction::special::template::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

impl From<Row> for journal::transaction::special::template::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
        }
    }
}
