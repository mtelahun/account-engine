pub mod array_string_24;
pub mod array_string_3;
pub mod array_string_64;
pub mod fixed_len_char;
pub mod ids;
pub mod sequence;

const STRING3_LEN: usize = 3;
const STRING24_LEN: usize = 24;
const STRING64_LEN: usize = 64;

// Re-export
pub use array_string_24::ArrayString24;
pub use array_string_3::ArrayString3;
pub use array_string_64::ArrayString64;
pub use fixed_len_char::FixedLenChar;
pub use ids::{JournalRefId, JournalTypeId};
pub use sequence::Sequence;
