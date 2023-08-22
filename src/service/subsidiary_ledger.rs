use async_trait::async_trait;

use crate::{
    domain::{AccountId, ExternalXactTypeCode, SubLedgerId},
    resource::{
        account_engine::AccountEngine,
        external,
        journal::AccountPostingRef,
        ledger::{self, journal_entry::LedgerKey},
        subsidiary_ledger,
    },
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
        + ResourceOperations<
            external::transaction_type::Model,
            external::transaction_type::ActiveModel,
            ExternalXactTypeCode,
        > + ResourceOperations<
            ledger::transaction::account::Model,
            ledger::transaction::account::ActiveModel,
            LedgerKey,
        > + Send
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

    async fn get_journal_entry_transaction_account(
        &self,
        posting_ref: &AccountPostingRef,
    ) -> Result<ledger::transaction::account::ActiveModel, ServiceError> {
        let xact = self.store().get(Some(&vec![posting_ref.key])).await?;
        if xact.is_empty() {
            return Err(ServiceError::EmptyRecord(format!(
                "account: {}, key: {}",
                posting_ref.account_id, posting_ref.key
            )));
        }

        Ok(xact[0])
    }

    async fn create_external_transaction_type(
        &self,
        model: &external::transaction_type::Model,
    ) -> Result<external::transaction_type::ActiveModel, ServiceError> {
        Ok(self.store().insert(model).await?)
    }

    async fn get_external_transaction_type(
        &self,
        ids: Option<&Vec<ExternalXactTypeCode>>,
    ) -> Result<Vec<external::transaction_type::ActiveModel>, ServiceError> {
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
