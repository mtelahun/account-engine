use postgres_types::{FromSql, ToSql};

use crate::domain::{ArrayLongString, LedgerId, SubLedgerId};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct Model {
    pub name: ArrayLongString,
    pub ledger_account_id: LedgerId,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub id: SubLedgerId,
    pub name: ArrayLongString,
    pub ledger_account_id: LedgerId,
}
