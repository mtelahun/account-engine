pub mod account_type;
pub mod array_code_string;
pub mod array_long_string;
pub mod array_short_string;
pub mod composite_ids;
pub mod entity_code;
pub mod external_xact_type_code;
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
pub use array_code_string::ArrayString3;
pub use array_long_string::ArrayString128;
pub use array_short_string::ArrayString24;
pub use composite_ids::{AccountTransactionId, JournalTransactionColumnId, JournalTransactionId};
pub use entity_code::EntityCode;
pub use external_xact_type_code::ExternalXactTypeCode;
pub use fixed_len_char::FixedLenChar;
pub use ids::{
    AccountId, ColumnTotalId, EntityId, GeneralLedgerId, JournalId, JournalRefId, JournalTypeId,
    LedgerId, PeriodId, SpecialJournalColId, SpecialJournalTemplateId, SubLedgerId,
    TemplateColumnId,
};
pub use ledger_xact_type_code::{LedgerXactTypeCode, XACT_ACCOUNT, XACT_LEDGER};
pub use sequence::Sequence;
pub use xact_type::XactType;
