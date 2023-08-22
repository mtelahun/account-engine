use postgres_types::{FromSql, ToSql};

use crate::domain::{
    ids::JournalId, ArrayLongString, ArrayShortString, LedgerId, SubJournalTemplateId,
};

pub mod transaction;
pub mod typ;

pub use transaction::{AccountPostingRef, LedgerPostingRef};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, FromSql, ToSql)]
#[postgres(name = "journaltype")]
pub enum JournalType {
    #[postgres(name = "general")]
    #[default]
    General,
    #[postgres(name = "special")]
    Special,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Model {
    pub name: ArrayLongString,
    pub code: ArrayShortString,
    pub journal_type: JournalType,
    pub ledger_id: Option<LedgerId>,
    pub template_id: Option<SubJournalTemplateId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: JournalId,
    pub name: ArrayLongString,
    pub code: ArrayShortString,
    pub journal_type: JournalType,
    pub ledger_id: Option<LedgerId>,
    pub template_id: Option<SubJournalTemplateId>,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: JournalId::new(),
            name: value.name,
            code: value.code,
            journal_type: value.journal_type,
            ledger_id: value.ledger_id,
            template_id: value.template_id,
        }
    }
}
