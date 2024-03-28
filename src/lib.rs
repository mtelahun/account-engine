pub mod domain;
pub mod infrastructure;
pub mod resource;
pub mod shared_kernel;

use async_trait::async_trait;
use domain::general_ledger::ledger_id::LedgerId;
use infrastructure::data::db_context::error::OrmError;
use resource::{
    accounting_period, journal,
    ledger::{self, transaction},
    organization,
};
use shared_kernel::{ArrayString24, JournalTransactionId, Sequence, SpecialJournalTemplateId};

#[async_trait]
pub trait Store {
    async fn create_schema(&self) -> Result<(), OrmError>;

    async fn organization(&self) -> Result<organization::ActiveModel, OrmError>;

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::general::line::ActiveModel,
    ) -> Result<u64, OrmError>;

    async fn find_ledger_by_no(
        &self,
        no: ArrayString24,
    ) -> Result<Option<ledger::ActiveModel>, OrmError>;

    async fn journal_entries_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError>;

    async fn journal_entry_ledgers_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError>;

    async fn find_journal_by_code<'a>(
        &self,
        journal_code: &str,
    ) -> Result<Vec<journal::ActiveModel>, OrmError>;

    async fn find_period_by_fiscal_year(
        &self,
        fy: i32,
    ) -> Result<Option<accounting_period::ActiveModel>, OrmError>;

    async fn get_journal_transaction_columns<'a>(
        &self,
        ids: &'a [JournalTransactionId],
        sequence: Sequence,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError>;

    async fn get_journal_transaction_template_columns(
        &self,
        id: SpecialJournalTemplateId,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError>;
}
