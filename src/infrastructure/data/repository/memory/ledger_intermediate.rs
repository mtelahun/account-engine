use async_trait::async_trait;

use crate::{
    domain::LedgerId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::ledger,
};

#[async_trait]
impl RepositoryOperations<ledger::intermediate::Model, ledger::intermediate::ActiveModel, LedgerId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &ledger::intermediate::Model,
    ) -> Result<ledger::intermediate::ActiveModel, OrmError> {
        let mut inner = self.inner.write().await;
        let intermediate = ledger::intermediate::ActiveModel { id: model.id };
        if inner.ledger_intermediate.contains_key(&intermediate.id) {
            return Err(OrmError::DuplicateRecord(model.id.to_string()));
        }
        inner
            .ledger_intermediate
            .insert(intermediate.id, intermediate);

        Ok(intermediate)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::intermediate::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::intermediate::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger_intermediate.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger_intermediate.values() {
                res.push(*account);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<ledger::intermediate::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::intermediate::ActiveModel) -> Result<u64, OrmError> {
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
