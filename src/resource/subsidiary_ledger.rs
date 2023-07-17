use arrayvec::ArrayString;

use crate::domain::{AccountBookId, AccountId};

pub struct Model {
    pub name: ArrayString<256>,
    pub ledger_account_id: AccountId,
}

pub struct ActiveModel {
    pub id: AccountBookId,
    pub name: ArrayString<256>,
    pub ledger_account_id: AccountId,
}
