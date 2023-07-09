use crate::domain::{AccountBookId, AccountId};

#[derive(Clone, Debug)]
pub struct Model {
    pub id: AccountId,
    pub subsidiary_ledger_id: AccountBookId,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub id: AccountId,
    pub subsidiary_ledger_id: AccountBookId,
}
