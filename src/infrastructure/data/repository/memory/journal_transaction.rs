use crate::{
    domain::JournalTransactionId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::journal,
};
use async_trait::async_trait;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::Model,
        journal::transaction::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::Model,
    ) -> Result<journal::transaction::ActiveModel, OrmError> {
        let jtx_record = journal::transaction::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let mut inner = self.inner.write().await;
        if inner.journal_xact.contains_key(&jtx_record.id()) {
            return Err(OrmError::Internal(format!(
                "db error: ERROR: duplicate key value violates unique constraint \
                \"journal_transaction_record_pkey\"\nDETAIL: Key (journal_id, \"timestamp\")=({}, {}) already exists.",
                model.journal_id, model.timestamp
            )));
        }
        inner.journal_xact.insert(jtx_record.id(), jtx_record);

        Ok(jtx_record)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::ActiveModel>::new();
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
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &journal::transaction::ActiveModel) -> Result<u64, OrmError> {
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
