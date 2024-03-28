use async_trait::async_trait;

use crate::{
    domain::{AccountId, ExternalXactTypeCode, SubLedgerId},
    infrastructure::data::db_context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::ResourceOperations,
    },
    resource::{
        account_engine::AccountEngine,
        external,
        journal::LedgerAccountPostingRef,
        ledger::{self, journal_entry::LedgerKey},
        subsidiary_ledger,
    },
    Store,
};

use super::{
    external::{ExternalAccount, ExternalAccountBuilder},
    ServiceError,
};

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
        builder: ExternalAccountBuilder,
    ) -> Result<ExternalAccount, ServiceError> {
        Ok(self.store().insert(&builder.to_model()).await?.into())
    }

    async fn get_accounts(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<external::account::ActiveModel>, ServiceError> {
        Ok(self.store().get(ids).await?)
    }

    async fn get_journal_entry_transaction_account(
        &self,
        posting_ref: &LedgerAccountPostingRef,
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
