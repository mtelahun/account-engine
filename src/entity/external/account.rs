use arrayvec::ArrayString;

use crate::domain::{AccountBookId, AccountId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub book_id: AccountBookId,
    pub account_no: ArrayString<64>,
    pub entity_type: super::entity_type::Model,
    pub account_type: super::account_type::Model,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: AccountId,
    pub book_id: AccountBookId,
    pub account_no: ArrayString<64>,
    pub entity_type: super::entity_type::Model,
    pub account_type: super::account_type::Model,
}
