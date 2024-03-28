use postgres_types::{FromSql, ToSql};

use crate::{
    domain::general_ledger::ledger_id::LedgerId,
    shared_kernel::{ArrayString128, SubLedgerId},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct Model {
    pub name: ArrayString128,
    pub ledger_id: LedgerId,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub id: SubLedgerId,
    pub name: ArrayString128,
    pub ledger_id: LedgerId,
}
