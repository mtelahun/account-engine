use postgres_types::{FromSql, ToSql};

use crate::{domain::LedgerId, resource::LedgerKey};

pub mod general;
pub mod record;
pub mod special;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "transactionstate")]
pub enum TransactionState {
    #[postgres(name = "pending")]
    #[default]
    Pending,
    #[postgres(name = "archived")]
    Archived,
    #[postgres(name = "posted")]
    Posted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionAccountType {
    Account,
    Ledger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct PostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) account_id: LedgerId,
}

impl std::fmt::Display for TransactionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            TransactionState::Pending => "Pending",
            TransactionState::Posted => "Posted",
            TransactionState::Archived => "Archived",
        };

        write!(f, "{}", state)
    }
}
