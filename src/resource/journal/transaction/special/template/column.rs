use crate::domain::{LedgerId, SubJournalTemplateColId, SubJournalTemplateId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Model {
    pub template_id: SubJournalTemplateId,
    pub sequence: usize,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: SubJournalTemplateColId,
    pub template_id: SubJournalTemplateId,
    pub sequence: usize,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: SubJournalTemplateColId::new(),
            template_id: value.template_id,
            sequence: value.sequence,
            cr_ledger_id: value.cr_ledger_id,
            dr_ledger_id: value.dr_ledger_id,
        }
    }
}
