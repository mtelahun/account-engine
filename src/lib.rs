use async_trait::async_trait;
use domain::{AccountId, ArrayShortString, JournalTransactionId};
use resource::{
    accounting_period, journal,
    ledger::{self, transaction},
    organization,
};
use store::OrmError;

#[async_trait]
pub trait Store {
    async fn create_schema(&self) -> Result<(), OrmError>;

    async fn organization(&self) -> Result<organization::ActiveModel, OrmError>;

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::line::ledger::ActiveModel,
    ) -> Result<u64, OrmError>;

    async fn find_ledger_by_no(
        &self,
        no: ArrayShortString,
    ) -> Result<Option<ledger::ActiveModel>, OrmError>;

    async fn journal_entries_by_ledger(
        &self,
        ids: &[AccountId],
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError>;

    async fn journal_entry_ledgers_by_ledger(
        &self,
        ids: &[AccountId],
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError>;

    async fn find_journal_by_code<'a>(
        &self,
        journal_code: &str,
    ) -> Result<Vec<journal::ActiveModel>, OrmError>;

    async fn find_period_by_fiscal_year(
        &self,
        fy: i32,
    ) -> Result<Option<accounting_period::ActiveModel>, OrmError>;
}

pub mod domain;
pub mod resource;
pub mod service;
pub mod store;
