#![allow(clippy::diverging_sub_expression)]
pub mod error;
pub mod external;
pub mod general_ledger;
pub mod journal;
pub mod journal_transaction;
pub mod period;
pub mod subsidiary_ledger;

// Re-exports
pub use error::ServiceError;
pub use general_ledger::ledger::{Ledger, LedgerAccount, LedgerService};
pub use general_ledger::service::GeneralLedgerService;
pub use journal::general::GeneralJournalService;
pub use journal::special::SpecialJournalService;
pub use journal_transaction::general::JournalTransactionService;
pub use journal_transaction::special::SpecialJournalTransactionService;
pub use period::service::AccountingPeriodService;
pub use subsidiary_ledger::service::SubsidiaryLedgerService;
