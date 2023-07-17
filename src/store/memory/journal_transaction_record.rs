use crate::{
    domain::JournalTransactionId,
    resource::journal,
    store::{OrmError, ResourceOperations},
};
use async_trait::async_trait;

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::record::Model,
        journal::transaction::record::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::record::Model,
    ) -> Result<journal::transaction::record::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        let jtx_record = journal::transaction::record::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let mut inner = self.inner.write().await;
        if inner.journal_xact.contains_key(&jtx_id) {
            return Err(OrmError::Internal(format!(
                "db error: ERROR: duplicate key value violates unique constraint \
                \"journal_transaction_record_pkey\"\nDETAIL: Key (journal_id, \"timestamp\")=({}, {}) already exists.",
                model.journal_id, model.timestamp
            )));
        }
        inner.journal_xact.insert(jtx_id, jtx_record);

        Ok(jtx_record)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::record::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::record::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.journal_xact.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.journal_xact.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::record::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::record::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}
