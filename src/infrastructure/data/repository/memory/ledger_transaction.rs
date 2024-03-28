use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::{ledger, LedgerKey},
};

#[async_trait]
impl ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &ledger::transaction::Model,
    ) -> Result<ledger::transaction::ActiveModel, OrmError> {
        let entry = ledger::transaction::ActiveModel {
            ledger_id: model.ledger_id,
            timestamp: model.timestamp,
            ledger_xact_type_code: model.ledger_xact_type_code,
            amount: model.amount,
            journal_ref: model.journal_ref,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner.journal_entry.iter().any(|(k, _)| *k == entry.id());
        if is_duplicate {
            return Err(OrmError::DuplicateRecord("duplicate journal entry".into()));
        }
        inner.journal_entry.insert(entry.id(), entry);

        Ok(entry)
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerKey>>,
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger::transaction::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for entry in inner.journal_entry.values() {
                if ids.iter().any(|i| *i == entry.id()) {
                    res.push(*entry);
                }
            }
        } else {
            for entry in inner.journal_entry.values() {
                res.push(*entry);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger::transaction::ActiveModel) -> Result<u64, OrmError> {
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
