use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
    shared_kernel::{JournalId, JournalTransactionId},
};
use async_trait::async_trait;

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::Model,
        journal::transaction::special::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::Model,
    ) -> Result<journal::transaction::special::ActiveModel, OrmError> {
        let jtx_record: journal::transaction::special::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        if inner.journal_xact_record_sub.contains_key(&jtx_record.id()) {
            return Err(OrmError::Internal(format!(
                "db error: ERROR: duplicate key value violates unique constraint \
                \"journal_transaction_record_pkey\"\nDETAIL: Key (journal_id, \"timestamp\")=({}, {}) already exists.",
                model.journal_id, model.timestamp
            )));
        }
        inner
            .journal_xact_record_sub
            .insert(jtx_record.id(), jtx_record);

        Ok(jtx_record)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.journal_xact_record_sub.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.journal_xact_record_sub.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        domain: &str,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, OrmError> {
        let j_id: Vec<&str> = domain.split('=').collect();
        let j_id = JournalId::parse_str(j_id[1].trim()).map_err(OrmError::Internal)?;
        let mut res = Vec::<journal::transaction::special::ActiveModel>::new();
        let inner = self.inner.read().await;
        for value in inner.journal_xact_record_sub.values() {
            if value.journal_id == j_id {
                res.push(*value);
            }
        }

        Ok(res)
    }

    async fn save(
        &self,
        model: &journal::transaction::special::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            inner.journal_xact_record_sub.entry(model.id())
        {
            e.insert(*model);

            Ok(1)
        } else {
            Err(OrmError::RecordNotFound(format!(
                "journal transaction: {}",
                model.id()
            )))
        }
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
