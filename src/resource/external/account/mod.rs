use chrono::{NaiveDate, NaiveDateTime};
use postgres_types::{FromSql, ToSql};

use crate::domain::{
    ids::{AccountId, EntityId},
    ArrayString128, ArrayString24, SubLedgerId,
};

pub mod transaction;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "subsidiaryledgerkey")]
pub struct SubsidiaryLedgerKey {
    pub account_id: AccountId,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub subledger_id: SubLedgerId,
    pub entity_id: EntityId,
    pub account_no: ArrayString24,
    pub name: ArrayString128,
    pub date_opened: NaiveDate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: AccountId,
    pub subledger_id: SubLedgerId,
    pub entity_id: EntityId,
    pub account_no: ArrayString24,
    pub name: ArrayString128,
    pub date_opened: NaiveDate,
}

impl std::fmt::Display for SubsidiaryLedgerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "SubsidiaryLedgerKey(id: {}, timestamp: {})",
            self.account_id, self.timestamp
        );
        write!(f, "{msg}")
    }
}
