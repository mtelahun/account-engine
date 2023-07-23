use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::{JournalId, LedgerId, SpecJournalTemplateColId};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub column_template_id: SpecJournalTemplateColId,
    pub amount: Decimal,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub column_template_id: SpecJournalTemplateColId,
    pub dr_ledger_id: Option<LedgerId>,
    pub cr_ledger_id: Option<LedgerId>,
    pub amount: Decimal,
}
