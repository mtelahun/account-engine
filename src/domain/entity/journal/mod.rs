use crate::shared_kernel::{ArrayString24, ArrayString64};

use self::journal_id::JournalId;

use super::{
    ledger::ledger_id::LedgerId,
    special_journal_template::special_journal_template_id::SpecialJournalTemplateId,
};

pub mod journal_id;
pub mod journal_type_code;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Journal {
    pub id: JournalId,
    pub name: ArrayString64,
    pub code: ArrayString24,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpecialJournal {
    id: JournalId,
    name: ArrayString64,
    code: ArrayString24,
    control_ledger_id: LedgerId,
    template_id: SpecialJournalTemplateId,
}

impl Journal {
    pub fn build_special_journal(
        self,
        control_ledger_id: LedgerId,
        template_id: SpecialJournalTemplateId,
    ) -> SpecialJournal {
        SpecialJournal {
            id: self.id,
            name: self.name,
            code: self.code,
            control_ledger_id,
            template_id,
        }
    }

    pub fn code(&self) -> ArrayString24 {
        self.code
    }

    pub fn id(&self) -> JournalId {
        self.id
    }

    pub fn name(&self) -> ArrayString64 {
        self.name
    }

    pub fn new(name: &str, code: &str) -> Journal {
        Journal {
            id: JournalId::new(),
            name: name.into(),
            code: code.into(),
        }
    }
}

impl SpecialJournal {
    pub fn code(&self) -> ArrayString24 {
        self.code
    }

    pub fn control_ledger_id(&self) -> LedgerId {
        self.control_ledger_id
    }

    pub fn id(&self) -> JournalId {
        self.id
    }

    pub fn name(&self) -> ArrayString64 {
        self.name
    }

    pub fn template_id(&self) -> SpecialJournalTemplateId {
        self.template_id
    }
}
