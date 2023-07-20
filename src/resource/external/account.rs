use chrono::NaiveDate;

use crate::domain::{entity_code::EntityCode, ArrayShortString, LedgerId, SubLedgerId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub subsidiary_ledger_id: SubLedgerId,
    pub entity_type_code: EntityCode,
    pub account_no: ArrayShortString,
    pub date_opened: NaiveDate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: LedgerId,
    pub subsidiary_ledger_id: SubLedgerId,
    pub entity_type_code: EntityCode,
    pub account_no: ArrayShortString,
    pub date_opened: NaiveDate,
}
