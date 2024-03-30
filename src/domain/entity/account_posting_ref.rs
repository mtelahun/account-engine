use postgres_types::{FromSql, ToSql};

use super::subsidiary_ledger::subsidiary_ledger_key::SubsidiaryLedgerKey;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "accountpostingref")]
pub struct AccountPostingRef {
    #[postgres(name = "key")]
    pub(crate) key: SubsidiaryLedgerKey,
}
