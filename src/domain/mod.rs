pub mod account_type;
pub mod array_code_string;
pub mod array_long_string;
pub mod array_short_string;
pub mod composite_ids;
pub mod entity_code;
pub mod external_xact_type_code;
pub mod fixed_len_char;
pub mod ids;
pub mod ledger_xact_type_code;
pub mod unique_id;
pub mod xact_type;

const DEFAULT_CODE_LEN: usize = 3;
const DEFAULT_LONGSTRING_LEN: usize = 256;
const DEFAULT_SHORTSTRING_LEN: usize = 24;

// Re-export
pub use array_code_string::ArrayCodeString;
pub use array_long_string::ArrayLongString;
pub use array_short_string::ArrayShortString;
pub use composite_ids::JournalTransactionId;
pub use external_xact_type_code::ExternalXactTypeCode;
pub use fixed_len_char::FixedLenChar;
pub use ids::{
    AccountId, GeneralLedgerId, JournalId, JournalTypeId, LedgerId, PeriodId, SpecJournalColId,
    SpecJournalTemplateColId, SpecJournalTemplateId, SubLedgerId,
};
pub use ledger_xact_type_code::{LedgerXactTypeCode, XACT_ACCOUNT, XACT_LEDGER};
pub use xact_type::XactType;
