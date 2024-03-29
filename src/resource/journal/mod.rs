use postgres_types::{FromSql, ToSql};

use crate::{
    domain::entity::{
        general_journal::journal_id::JournalId, ledger::ledger_id::LedgerId,
        special_journal_template::special_journal_template_id::SpecialJournalTemplateId,
    },
    shared_kernel::{ArrayString24, ArrayString64},
};

pub mod transaction;
pub mod typ;

// Re-exports
pub use transaction::{
    special::JournalTransactionSpecial, LedgerAccountPostingRef, LedgerPostingRef,
};

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
    pub name: ArrayString64,
    pub code: ArrayString24,
    pub journal_type: JournalType,
    pub control_ledger_id: Option<LedgerId>,
    pub template_id: Option<SpecialJournalTemplateId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: JournalId,
    pub name: ArrayString64,
    pub code: ArrayString24,
    pub journal_type: JournalType,
    pub control_ledger_id: Option<LedgerId>,
    pub template_id: Option<SpecialJournalTemplateId>,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: JournalId::new(),
            name: value.name,
            code: value.code,
            journal_type: value.journal_type,
            control_ledger_id: value.control_ledger_id,
            template_id: value.template_id,
        }
    }
}
