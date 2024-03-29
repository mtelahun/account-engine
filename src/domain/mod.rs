#![allow(clippy::diverging_sub_expression)]
pub mod entity;
pub mod error;
pub mod external;
pub mod general_journal;
pub mod general_ledger;
pub mod journal_transaction;
pub mod special_journal;
pub mod subsidiary_ledger;

// Re-exports
pub use entity::ledger::{Ledger, LedgerAccount, LedgerService};
pub use entity::period::service::AccountingPeriodService;
pub use error::ServiceError;
pub use general_journal::service::GeneralJournalService;
pub use general_ledger::service::GeneralLedgerService;
pub use journal_transaction::general::JournalTransactionService;
pub use journal_transaction::special::SpecialJournalTransactionService;
pub use special_journal::service::SpecialJournalService;
pub use subsidiary_ledger::service::SubsidiaryLedgerService;
