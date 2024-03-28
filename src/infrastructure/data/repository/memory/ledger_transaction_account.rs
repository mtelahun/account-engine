use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::{ledger::transaction, LedgerKey},
};

#[async_trait]
impl ResourceOperations<transaction::account::Model, transaction::account::ActiveModel, LedgerKey>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &transaction::account::Model,
    ) -> Result<transaction::account::ActiveModel, OrmError> {
        let xact = transaction::account::ActiveModel {
            ledger_id: model.ledger_id,
            timestamp: model.timestamp,
            account_id: model.account_id,
            xact_type_code: model.xact_type_code,
            xact_type_external_code: model.xact_type_external_code,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .ledger_xact_account
            .iter()
            .any(|(k, v)| (*k == xact.id()) && (v.account_id == xact.account_id));
        if is_duplicate {
            return Err(OrmError::DuplicateRecord(
                "duplicate account transaction".into(),
            ));
        }
        inner.ledger_xact_account.insert(xact.id(), xact);

        Ok(xact)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<transaction::account::ActiveModel>, OrmError> {
        let mut res = Vec::<transaction::account::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for xact in inner.ledger_xact_account.values() {
                if ids.iter().any(|i| *i == xact.id()) {
                    res.push(*xact);
                }
            }
        } else {
            for xact in inner.ledger_xact_account.values() {
                res.push(*xact);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<transaction::account::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &transaction::account::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerKey) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: LedgerKey) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: LedgerKey) -> Result<u64, OrmError> {
        todo!()
    }
}
