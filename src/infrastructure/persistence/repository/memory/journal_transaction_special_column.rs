use async_trait::async_trait;

use crate::{
    domain::entity::general_journal_transaction::journal_transaction_id::JournalTransactionId,
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::column::Model,
        journal::transaction::special::column::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::column::Model,
    ) -> Result<journal::transaction::special::column::ActiveModel, OrmError> {
        let jtx_line_account: journal::transaction::special::column::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        match inner
            .journal_xact_special_column
            .get_mut(&jtx_line_account.id())
        {
            Some(val) => val.push(jtx_line_account),
            None => {
                inner
                    .journal_xact_special_column
                    .insert(jtx_line_account.id(), vec![jtx_line_account]);
            }
        };

        Ok(jtx_line_account)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::column::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_special_column.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_special_column.values() {
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
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &journal::transaction::special::column::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.journal_xact_special_column.get_mut(&model.id()) {
            Some(val) => {
                for v in val.iter_mut() {
                    if v.sequence == model.sequence {
                        let _ = std::mem::replace(v, *model);
                    }
                }

                return Ok(1);
            }
            None => {
                return Err(OrmError::RecordNotFound(format!(
                    "journal transaction subsidiary colum sequence: {}",
                    model.sequence
                )))
            }
        };
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
