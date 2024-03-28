use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::{ledger::transaction, LedgerKey},
};

#[async_trait]
impl RepositoryOperations<transaction::ledger::Model, transaction::ledger::ActiveModel, LedgerKey>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &transaction::ledger::Model,
    ) -> Result<transaction::ledger::ActiveModel, OrmError> {
        let xact = transaction::ledger::ActiveModel {
            ledger_id: model.ledger_id,
            timestamp: model.timestamp,
            ledger_dr_id: model.ledger_dr_id,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .ledger_xact_ledger
            .iter()
            .any(|(k, v)| (*k == xact.id()) && (v.ledger_dr_id == xact.ledger_dr_id));
        if is_duplicate {
            return Err(OrmError::DuplicateRecord(
                "duplicate ledger transaction".into(),
            ));
        }
        inner.ledger_xact_ledger.insert(xact.id(), xact);

        Ok(xact)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError> {
        let mut res = Vec::<transaction::ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for xact in inner.ledger_xact_ledger.values() {
                if ids.iter().any(|i| *i == xact.id()) {
                    res.push(*xact);
                }
            }
        } else {
            for xact in inner.ledger_xact_ledger.values() {
                res.push(*xact);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &transaction::ledger::ActiveModel) -> Result<u64, OrmError> {
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
