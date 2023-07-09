use async_trait::async_trait;
use domain::{AccountId, JournalTransactionId};
use entity::{
    accounting_period, journal,
    ledger::{self, transaction},
    organization, LedgerKey,
};
use resource::OrmError;

#[async_trait]
pub trait Repository {
    async fn create_schema(&self) -> Result<(), OrmError>;

    async fn organization(&self) -> Result<organization::ActiveModel, OrmError>;

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::line::ledger::ActiveModel,
    ) -> Result<u64, OrmError>;

    async fn find_ledger_by_model(
        &self,
        model: &ledger::Model,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError>;

    async fn find_ledger_line(
        &self,
        ids: &Option<Vec<LedgerKey>>,
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError>;

    async fn find_ledger_transaction(
        &self,
        ids: &Option<Vec<LedgerKey>>,
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError>;

    async fn ledger_line_by_key(&self, key: LedgerKey) -> Option<ledger::transaction::ActiveModel>;

    async fn ledger_transactions_by_ledger_id(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger::transaction::ActiveModel>;

    async fn ledger_transaction_by_dr(
        &self,
        account_id: AccountId,
    ) -> Vec<transaction::ledger::ActiveModel>;

    async fn find_journal_by_code<'a>(
        &self,
        journal_code: &str,
    ) -> Result<Vec<journal::ActiveModel>, OrmError>;

    async fn find_period_by_year(
        &self,
        model: &accounting_period::Model,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError>;
}

pub mod domain;
pub mod entity;
// pub mod memory_store;
// pub mod postgres;
pub mod resource;
pub mod service;
