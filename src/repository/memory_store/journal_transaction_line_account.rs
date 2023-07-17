use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    repository::{OrmError, ResourceOperations},
    resource::journal,
};

use super::repository::MemoryRepository;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::line::account::Model,
        journal::transaction::line::account::ActiveModel,
        JournalTransactionId,
    > for MemoryRepository
{
    async fn insert(
        &self,
        model: &journal::transaction::line::account::Model,
    ) -> Result<journal::transaction::line::account::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        let jtx_line_account = journal::transaction::line::account::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            account_id: model.account_id,
            xact_type: model.xact_type,
            amount: model.amount,
            state: model.state,
            posting_ref: model.posting_ref,
        };
        let mut inner = self.inner.write().await;
        inner
            .journal_xact_line_account
            .insert(jtx_id, jtx_line_account);

        Ok(jtx_line_account)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::line::account::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::line::account::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.journal_xact_line_account.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.journal_xact_line_account.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::line::account::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::line::account::ActiveModel,
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
