#![allow(clippy::diverging_sub_expression)]
pub mod error;
pub mod external;
pub mod general_ledger;
pub mod journal;
pub mod journal_transaction;
pub mod ledger;
pub mod period;
pub mod subsidiary_ledger;

// Re-exports
pub use error::ServiceError;
pub use general_ledger::GeneralLedgerService;
pub use journal::general::GeneralJournalService;
pub use journal::special::SpecialJournalService;
pub use journal_transaction::general::JournalTransactionService;
pub use journal_transaction::special::SpecialJournalTransactionService;
pub use ledger::{Ledger, LedgerAccount, LedgerService};
pub use period::AccountingPeriodService;
pub use subsidiary_ledger::SubsidiaryLedgerService;
