use postgres_types::{FromSql, ToSql};

use crate::{
    domain::{AccountId, LedgerId},
    resource::LedgerKey,
};

pub mod general;
pub mod record;
pub mod reference;
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
pub struct LedgerPostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) ledger_id: LedgerId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct AccountPostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) account_id: AccountId,
}

impl LedgerPostingRef {
    pub fn new(key: LedgerKey, ledger_id: LedgerId) -> Self {
        Self { key, ledger_id }
    }

    pub fn key(&self) -> LedgerKey {
        self.key
    }

    pub fn ledger_id(&self) -> LedgerId {
        self.ledger_id
    }
}

impl AccountPostingRef {
    pub fn new(key: LedgerKey, account_id: AccountId) -> Self {
        Self { key, account_id }
    }

    pub fn key(&self) -> LedgerKey {
        self.key
    }

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }
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
