use crate::domain::{LedgerId, SubLedgerId};

#[derive(Clone, Debug)]
pub struct Model {
    pub id: LedgerId,
    pub subsidiary_ledger_id: SubLedgerId,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub id: LedgerId,
    pub subsidiary_ledger_id: SubLedgerId,
}
