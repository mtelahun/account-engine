use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    resource::journal,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::line::Model,
        journal::transaction::special::line::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::line::Model,
    ) -> Result<journal::transaction::special::line::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        let jtx_line_account = model.into();
        let mut inner = self.inner.write().await;
        match inner.journal_xact_special.get_mut(&jtx_id) {
            Some(val) => val.push(jtx_line_account),
            None => {
                inner
                    .journal_xact_special
                    .insert(jtx_id, vec![jtx_line_account]);
            }
        };

        Ok(jtx_line_account)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::line::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::line::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_special.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_special.values() {
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
    ) -> Result<Vec<journal::transaction::special::line::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::line::ActiveModel,
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
