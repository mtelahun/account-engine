use postgres_types::{FromSql, ToSql};

use crate::domain::{ids::JournalId, ArrayLongString, ArrayShortString};

pub mod transaction;
pub mod typ;

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, FromSql, ToSql)]
#[postgres(name = "journaltype")]
pub enum JournalType {
    #[postgres(name = "general")]
    #[default]
    General,
    #[postgres(name = "special")]
    Special,
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Model {
    pub name: ArrayLongString,
    pub code: ArrayShortString,
    pub journal_type: JournalType,
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: JournalId,
    pub name: ArrayLongString,
    pub code: ArrayShortString,
    pub journal_type: JournalType,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: JournalId::new(),
            name: value.name,
            code: value.code,
            journal_type: value.journal_type,
        }
    }
}
