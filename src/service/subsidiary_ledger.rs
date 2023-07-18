use async_trait::async_trait;

use crate::{
    domain::{AccountId, SubLedgerId},
    resource::{account_engine::AccountEngine, external, subsidiary_ledger},
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Store,
};

use super::ServiceError;

#[async_trait]
pub trait SubsidiaryLedgerService<R>
where
    R: Store
        + ResourceOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn create_subsidiary_ledger(
        &self,
        model: &subsidiary_ledger::Model,
    ) -> Result<subsidiary_ledger::ActiveModel, ServiceError> {
        Ok(self.store().insert(model).await?)
    }

    async fn get_subsidiary_ledgers(
        &self,
        ids: Option<&Vec<SubLedgerId>>,
    ) -> Result<Vec<subsidiary_ledger::ActiveModel>, ServiceError> {
        Ok(self.store().get(ids).await?)
    }

    async fn create_account(
        &self,
        model: &external::account::Model,
    ) -> Result<external::account::ActiveModel, ServiceError> {
        Ok(self.store().insert(model).await?)
    }

    async fn get_accounts(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<external::account::ActiveModel>, ServiceError> {
        Ok(self.store().get(ids).await?)
    }
}

impl SubsidiaryLedgerService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}

impl SubsidiaryLedgerService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}