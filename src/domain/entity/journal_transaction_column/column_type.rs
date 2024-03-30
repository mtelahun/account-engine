use postgres_types::{FromSql, ToSql};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "specialcolumntype")]
pub enum JournalTransactionColumnType {
    #[postgres(name = "ledger_drcr")]
    LedgerDrCr,
    #[postgres(name = "text")]
    Text,
    #[postgres(name = "account_dr")]
    AccountDr,
    #[postgres(name = "account_cr")]
    AccountCr,
    #[postgres(name = "ledger_dr")]
    LedgerDr,
    #[postgres(name = "ledger_cr")]
    LedgerCr,
}
