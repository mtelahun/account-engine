use postgres_types::{FromSql, ToSql};

use crate::domain::{
    array_long_string::ArrayLongString, array_short_string::ArrayShortString, ArrayCodeString,
    LedgerId,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "ledgertype")]
pub enum LedgerType {
    #[postgres(name = "derived")]
    Derived,
    #[postgres(name = "intermediate")]
    Intermediate,
    #[postgres(name = "leaf")]
    Leaf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub ledger_no: ArrayShortString,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayLongString,
    pub currency_code: Option<ArrayCodeString>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: LedgerId,
    pub ledger_no: ArrayShortString,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayLongString,
    pub currency_code: Option<ArrayCodeString>,
}

pub mod derived;
pub mod intermediate;
pub mod journal_entry;
pub mod leaf;
pub mod transaction;
