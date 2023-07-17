use chrono::NaiveDateTime;

use crate::domain::{xact_type::XactType, AccountId};

use super::external_xact_type;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub ledger_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub xact_type: XactType,
    pub external_xact_type: external_xact_type::ActiveModel,
    pub account_id: AccountId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub ledger_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub xact_type: XactType,
    pub external_xact_type: external_xact_type::ActiveModel,
    pub account_no: AccountId,
}
