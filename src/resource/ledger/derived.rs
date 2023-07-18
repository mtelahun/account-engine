use crate::domain::{AccountId, SubLedgerId};

#[derive(Clone, Debug)]
pub struct Model {
    pub id: AccountId,
    pub subsidiary_ledger_id: SubLedgerId,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub id: AccountId,
    pub subsidiary_ledger_id: SubLedgerId,
}
