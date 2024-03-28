use postgres_types::{FromSql, ToSql};

use crate::shared_kernel::{
    array_long_string::ArrayString128, array_short_string::ArrayString24, ArrayString3, LedgerId,
};

pub mod derived;
pub mod intermediate;
pub mod journal_entry;
pub mod leaf;
pub mod transaction;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub number: ArrayString24,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayString128,
    pub currency_code: Option<ArrayString3>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: LedgerId,
    pub number: ArrayString24,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayString128,
    pub currency_code: Option<ArrayString3>,
}

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
