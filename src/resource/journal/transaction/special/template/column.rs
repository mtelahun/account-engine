use crate::domain::{LedgerId, SpecJournalTemplateColId, SpecJournalTemplateId};

#[derive(Clone, Copy, Debug)]
pub struct SpecColumnTemplate {
    pub dr_ledger_id: Option<LedgerId>,
    pub cr_ledger_id: Option<LedgerId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Model {
    pub template_id: SpecJournalTemplateId,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: SpecJournalTemplateColId,
    pub template_id: SpecJournalTemplateId,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
}
