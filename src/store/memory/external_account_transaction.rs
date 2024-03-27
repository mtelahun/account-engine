use async_trait::async_trait;

use crate::{
    domain::AccountTransactionId,
    resource::external,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        external::account::transaction::Model,
        external::account::transaction::ActiveModel,
        AccountTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &external::account::transaction::Model,
    ) -> Result<external::account::transaction::ActiveModel, OrmError> {
        let tx: external::account::transaction::ActiveModel = (*model).into();
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .external_account_transaction
            .iter()
            .any(|(k, _)| *k == tx.id());
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate account number: {}",
                tx.id()
            )));
        }
        inner.external_account_transaction.insert(tx.id(), tx);

        Ok(tx)
    }

    async fn get(
        &self,
        ids: Option<&Vec<AccountTransactionId>>,
    ) -> Result<Vec<external::account::transaction::ActiveModel>, OrmError> {
        let mut res = Vec::<external::account::transaction::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.external_account_transaction.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.external_account_transaction.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::account::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &external::account::transaction::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        let exists = inner
            .external_account_transaction
            .iter()
            .any(|(k, _)| *k == model.id());
        if exists {
            inner
                .external_account_transaction
                .insert(model.id(), *model);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "external account transaction: {}",
            model.id()
        )));
    }

    async fn delete(&self, id: AccountTransactionId) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.external_account_transaction.remove(&id) {
            Some(_) => return Ok(1),
            None => {
                return Err(OrmError::RecordNotFound(format!(
                    "extern account transaction: {id}"
                )))
            }
        }
    }

    async fn archive(&self, _id: AccountTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: AccountTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}
