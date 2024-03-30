use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::domain::entity::external_account::account_id::AccountId;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "subsidiaryledgerkey")]
pub struct SubsidiaryLedgerKey {
    pub account_id: AccountId,
    pub timestamp: NaiveDateTime,
}
