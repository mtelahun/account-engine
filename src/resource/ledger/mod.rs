use postgres_types::{FromSql, ToSql};

use crate::{
    domain::general_ledger::ledger_id::LedgerId,
    shared_kernel::{
        array_string_24::ArrayString24, array_string_64::ArrayString128, ArrayString3,
    },
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
