pub mod account_transaction_id;
pub mod array_string_24;
pub mod array_string_3;
pub mod array_string_64;
pub mod entity_id;
pub mod fixed_len_char;
pub mod ids;
pub mod journal_id;
pub mod journal_transaction_column_id;
pub mod journal_transaction_id;
pub mod journal_type_code;
pub mod ledger_xact_type_code;
pub mod sequence;
pub mod unique_id;
pub mod xact_type;

const STRING3_LEN: usize = 3;
const STRING24_LEN: usize = 24;
const STRING64_LEN: usize = 64;

// Re-export
pub use account_transaction_id::AccountTransactionId;
pub use array_string_24::ArrayString24;
pub use array_string_3::ArrayString3;
pub use array_string_64::ArrayString128;
pub use entity_id::EntityId;
pub use fixed_len_char::FixedLenChar;
pub use ids::{JournalRefId, JournalTypeId};
pub use journal_id::JournalId;
pub use journal_transaction_id::JournalTransactionId;
pub use ledger_xact_type_code::{LedgerXactTypeCode, XACT_ACCOUNT, XACT_LEDGER};
pub use sequence::Sequence;
pub use xact_type::XactType;
