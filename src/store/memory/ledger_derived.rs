use async_trait::async_trait;

use crate::{
    domain::LedgerId,
    resource::ledger,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl ResourceOperations<ledger::derived::Model, ledger::derived::ActiveModel, LedgerId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &ledger::derived::Model,
    ) -> Result<ledger::derived::ActiveModel, OrmError> {
        let mut inner = self.inner.write().await;
        let derived = ledger::derived::ActiveModel { id: model.id };
        if inner.ledger_derived.contains_key(&derived.id) {
            return Err(OrmError::DuplicateRecord(model.id.to_string()));
        }
        inner.ledger_derived.insert(derived.id, derived);

        Ok(derived)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::derived::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::derived::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger_derived.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger_derived.values() {
                res.push(*account);
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::derived::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::derived::ActiveModel) -> Result<u64, OrmError> {
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
