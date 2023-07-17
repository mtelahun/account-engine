use async_trait::async_trait;

use crate::{
    domain::AccountId,
    resource::ledger,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, AccountId> for MemoryStore {
    async fn insert(
        &self,
        model: &ledger::leaf::Model,
    ) -> Result<ledger::leaf::ActiveModel, OrmError> {
        let mut inner = self.inner.write().await;
        let leaf = ledger::leaf::ActiveModel { id: model.id };
        if inner.ledger_intermediate.contains_key(&leaf.id) {
            return Err(OrmError::DuplicateRecord(model.id.to_string()));
        }
        inner.ledger_account.insert(leaf.id, leaf);

        Ok(leaf)
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger::leaf::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::leaf::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger_account.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger_account.values() {
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

    async fn delete(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }
}
