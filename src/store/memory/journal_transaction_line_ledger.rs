use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    resource::journal,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::line::ledger::Model,
        journal::transaction::line::ledger::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::line::ledger::Model,
    ) -> Result<journal::transaction::line::ledger::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        let jtx_line = journal::transaction::line::ledger::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            ledger_id: model.ledger_id,
            xact_type: model.xact_type,
            amount: model.amount,
            state: model.state,
            posting_ref: model.posting_ref,
        };
        let mut inner = self.inner.write().await;
        match inner.journal_xact_line.get_mut(&jtx_id) {
            Some(val) => val.push(jtx_line),
            None => {
                inner.journal_xact_line.insert(jtx_id, vec![jtx_line]);
            }
        };

        Ok(jtx_line)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::line::ledger::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::line::ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_line.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_line.values() {
                for v in lst.iter() {
                    res.push(*v)
                }
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::line::ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::line::ledger::ActiveModel,
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
