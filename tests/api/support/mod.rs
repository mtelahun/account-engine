pub mod memstore;
pub mod pgstore;
pub mod service_test_interface;
pub mod special_journal_transaction;
pub mod state_interface;
pub mod test_column;
pub mod utils;

pub use special_journal_transaction::CreateLedgerType;

const ENTITYTYPE_PERSON: &str = "PE";
