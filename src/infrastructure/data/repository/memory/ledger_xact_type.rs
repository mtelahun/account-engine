use crate::{
    domain::ledger_xact_type_code::LedgerXactTypeCode,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::ledger_xact_type,
};
use async_trait::async_trait;

#[async_trait]
impl
    RepositoryOperations<ledger_xact_type::Model, ledger_xact_type::ActiveModel, LedgerXactTypeCode>
    for MemoryStore
{
    async fn insert(
        &self,
        _model: &ledger_xact_type::Model,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        todo!()
    }

    async fn get(
        &self,
        ids: Option<&Vec<LedgerXactTypeCode>>,
    ) -> Result<Vec<ledger_xact_type::ActiveModel>, OrmError> {
        let mut res = Vec::<ledger_xact_type::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.ledger_xact_type.values() {
                if ids.iter().any(|id| *id == value.code) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.ledger_xact_type.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger_xact_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &ledger_xact_type::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: LedgerXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }
}
