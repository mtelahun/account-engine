use async_trait::async_trait;

use crate::{
    domain::AccountId,
    repository::{OrmError, ResourceOperations},
    resource::ledger,
};

use super::repository::MemoryRepository;

#[async_trait]
impl ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId> for MemoryRepository {
    async fn insert(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let ledger = ledger::ActiveModel {
            id: AccountId::new(),
            ledger_no: model.ledger_no,
            ledger_type: model.ledger_type,
            parent_id: model.parent_id,
            name: model.name,
            currency_code: model.currency_code,
        };
        let mut inner = self.inner.write().await;
        if inner
            .ledger
            .iter()
            .any(|(_k, v)| v.ledger_no == ledger.ledger_no)
        {
            return Err(OrmError::DuplicateRecord(format!(
                "account {}",
                ledger.ledger_no
            )));
        }
        inner.ledger.insert(ledger.id, ledger);

        Ok(ledger)
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
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
