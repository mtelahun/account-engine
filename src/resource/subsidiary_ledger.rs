use postgres_types::{FromSql, ToSql};

use crate::{
    domain::entity::{ledger::ledger_id::LedgerId, subsidiary_ledger::subleder_id::SubLedgerId},
    shared_kernel::ArrayString64,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct Model {
    pub name: ArrayString64,
    pub ledger_id: LedgerId,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub id: SubLedgerId,
    pub name: ArrayString64,
    pub ledger_id: LedgerId,
}
