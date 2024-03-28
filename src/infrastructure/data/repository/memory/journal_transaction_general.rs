use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::journal,
};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::general::line::Model,
        journal::transaction::general::line::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::general::line::Model,
    ) -> Result<journal::transaction::general::line::ActiveModel, OrmError> {
        let jtx_line: journal::transaction::general::line::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        match inner.journal_xact_general.get_mut(&jtx_line.id()) {
            Some(val) => val.push(jtx_line),
            None => {
                inner
                    .journal_xact_general
                    .insert(jtx_line.id(), vec![jtx_line]);
            }
        };

        Ok(jtx_line)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::general::line::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::general::line::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_general.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_general.values() {
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
    ) -> Result<Vec<journal::transaction::general::line::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::general::line::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}
