use rust_decimal::Decimal;

use crate::domain::{ids::JournalId, ArrayLongString, LedgerId};

pub mod column;
pub mod line;
pub mod template;

#[derive(Clone, Copy, Debug)]
pub struct PostColumn {
    pub spec_journal_id: JournalId,
    pub name: ArrayLongString,
    pub dr_ledger_id: Option<LedgerId>,
    pub cr_ledger_id: Option<LedgerId>,
    pub amount: Decimal,
}
