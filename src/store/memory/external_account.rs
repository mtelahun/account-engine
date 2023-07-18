use async_trait::async_trait;

use crate::{
    domain::AccountId,
    resource::external,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &external::account::Model,
    ) -> Result<external::account::ActiveModel, OrmError> {
        let id = AccountId::new();
        let account = external::account::ActiveModel {
            id,
            subsidiary_ledger_id: model.subsidiary_ledger_id,
            entity_code: model.entity_code,
            account_no: model.account_no,
            date_opened: model.date_opened,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .external_account
            .iter()
            .any(|(k, v)| *k == id || (*v.account_no == model.account_no));
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate account number: {}",
                model.account_no
            )));
        }
        inner.external_account.insert(id, account);

        Ok(account)
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<external::account::ActiveModel>, OrmError> {
        let mut res = Vec::<external::account::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.external_account.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.external_account.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<external::account::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::account::ActiveModel) -> Result<u64, OrmError> {
        let account = external::account::ActiveModel {
            id: model.id,
            subsidiary_ledger_id: model.subsidiary_ledger_id,
            entity_code: model.entity_code,
            account_no: model.account_no,
            date_opened: model.date_opened,
        };
        let mut inner = self.inner.write().await;
        let exists = inner.external_account.iter().any(|(k, _)| *k == model.id);
        if exists {
            inner.external_account.insert(model.id, account);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "external account: ({}): {}",
            model.account_no, model.id
        )));
    }

    async fn delete(&self, id: AccountId) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.external_account.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("account id: {id}"))),
        }
    }

    async fn archive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: AccountId) -> Result<u64, OrmError> {
        todo!()
    }
}
