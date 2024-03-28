pub mod account_engine;
pub mod account_type;
pub mod accounting_period;
pub mod external;
pub mod general_ledger;
pub mod journal;
pub mod ledger;
pub mod ledger_xact_type;
pub mod organization;
pub mod subsidiary_ledger;

// Re-exports
pub use accounting_period::interim_period::InterimType;
pub use external::account::SubsidiaryLedgerKey;
pub use journal::transaction::{LedgerPostingRef, TransactionState};
pub use ledger::journal_entry::LedgerKey;
pub use ledger::LedgerType;
use rust_decimal::Decimal;

use crate::shared_kernel::XactType;

pub struct AccountBalance {
    pub amount: Decimal,
    pub xact_type: XactType,
}
