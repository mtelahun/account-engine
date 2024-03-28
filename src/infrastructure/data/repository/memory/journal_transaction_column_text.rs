use async_trait::async_trait;

use crate::{
    domain::composite_ids::JournalTransactionColumnId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::journal,
};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::column::text::Model,
        journal::transaction::column::text::ActiveModel,
        JournalTransactionColumnId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::column::text::Model,
    ) -> Result<journal::transaction::column::text::ActiveModel, OrmError> {
        let jtx_col: journal::transaction::column::text::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_text.get_mut(&jtx_col.id()) {
            Some(val) => val.push(jtx_col),
            None => {
                inner
                    .journal_xact_column_text
                    .insert(jtx_col.id(), vec![jtx_col]);
            }
        };

        Ok(jtx_col)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionColumnId>>,
    ) -> Result<Vec<journal::transaction::column::text::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::column::text::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_column_text.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_column_text.values() {
                for v in lst.iter() {
                    res.push(*v)
                }
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::column::text::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &journal::transaction::column::text::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_text.get_mut(&model.id()) {
            Some(val) => {
                for v in val.iter_mut() {
                    if v.id() == model.id() {
                        let _ = std::mem::replace(v, *model);
                    }
                }

                return Ok(1);
            }
            None => {
                return Err(OrmError::RecordNotFound(format!(
                    "journal transaction text colum id: {}",
                    model.id()
                )))
            }
        };
    }

    async fn delete(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }
}
