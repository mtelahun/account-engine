pub mod array_string_24;
pub mod array_string_3;
pub mod array_string_64;
pub mod composite_ids;
pub mod entity_id;
pub mod fixed_len_char;
pub mod ids;
pub mod journal_type_code;
pub mod ledger_xact_type_code;
pub mod sequence;
pub mod unique_id;
pub mod xact_type;

const STRING3_LEN: usize = 3;
const STRING24_LEN: usize = 24;
const STRING64_LEN: usize = 64;

// Re-export
pub use array_string_24::ArrayString24;
pub use array_string_3::ArrayString3;
pub use array_string_64::ArrayString128;
pub use composite_ids::{AccountTransactionId, JournalTransactionColumnId, JournalTransactionId};
pub use entity_id::EntityId;
pub use fixed_len_char::FixedLenChar;
pub use ids::{
    ColumnTotalId, JournalId, JournalRefId, JournalTypeId, SpecialJournalColId,
    SpecialJournalTemplateId, TemplateColumnId,
};
pub use ledger_xact_type_code::{LedgerXactTypeCode, XACT_ACCOUNT, XACT_LEDGER};
pub use sequence::Sequence;
pub use xact_type::XactType;
