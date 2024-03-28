use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::ledger,
    shared_kernel::LedgerId,
};

#[async_trait]
impl RepositoryOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &ledger::leaf::Model,
    ) -> Result<ledger::leaf::ActiveModel, OrmError> {
        let mut inner = self.inner.write().await;
        let leaf = ledger::leaf::ActiveModel { id: model.id };
        if inner.ledger_intermediate.contains_key(&leaf.id) {
            return Err(OrmError::DuplicateRecord(model.id.to_string()));
        }
        inner.ledger_leaf.insert(leaf.id, leaf);

        Ok(leaf)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::leaf::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::leaf::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger_leaf.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger_leaf.values() {
                res.push(*account);
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::leaf::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::leaf::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: LedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: LedgerId) -> Result<u64, OrmError> {
        todo!()
    }
}
