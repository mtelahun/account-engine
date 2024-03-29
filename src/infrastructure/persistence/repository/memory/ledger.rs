use async_trait::async_trait;

use crate::{
    domain::entity::ledger::ledger_id::LedgerId,
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::ledger,
};

#[async_trait]
impl RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId> for MemoryStore {
    async fn insert(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let ledger = ledger::ActiveModel {
            id: LedgerId::new(),
            number: model.number,
            ledger_type: model.ledger_type,
            parent_id: model.parent_id,
            name: model.name,
            currency_code: model.currency_code,
        };
        let mut inner = self.inner.write().await;
        if inner.ledger.iter().any(|(_k, v)| v.number == ledger.number) {
            return Err(OrmError::DuplicateRecord(format!(
                "account {}",
                ledger.number
            )));
        }
        inner.ledger.insert(ledger.id, ledger);

        Ok(ledger)
    }

    async fn get(&self, ids: Option<&Vec<LedgerId>>) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger.values() {
                res.push(*account);
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::ActiveModel) -> Result<u64, OrmError> {
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
