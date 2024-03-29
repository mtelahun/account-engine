use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::{
    domain::entity::{
        external_account::account_id::AccountId, general_journal::journal_id::JournalId,
        general_journal_transaction::journal_transaction_id::JournalTransactionId,
        ledger::ledger_id::LedgerId,
    },
    resource::{LedgerKey, SubsidiaryLedgerKey},
    shared_kernel::ArrayString64,
};

pub mod column;
pub mod general;
pub mod reference;
pub mod special;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString64,
}

#[derive(Clone, Copy, Debug, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct LedgerPostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) ledger_id: LedgerId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct LedgerAccountPostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) account_id: AccountId,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "accountpostingref")]
pub struct AccountPostingRef {
    #[postgres(name = "key")]
    pub(crate) key: SubsidiaryLedgerKey,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "transactionstate")]
pub enum TransactionState {
    #[postgres(name = "pending")]
    #[default]
    Pending,
    #[postgres(name = "posted")]
    Posted,
    #[postgres(name = "archived")]
    Archived,
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionAccountType {
    Account,
    Ledger,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
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

impl LedgerAccountPostingRef {
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

impl AccountPostingRef {
    pub fn new(account_id: &AccountId, timestamp: NaiveDateTime) -> Self {
        Self {
            key: SubsidiaryLedgerKey {
                account_id: *account_id,
                timestamp,
            },
        }
    }
}

impl std::fmt::Display for TransactionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            TransactionState::Pending => "pending",
            TransactionState::Posted => "posted",
            TransactionState::Archived => "archived",
        };

        write!(f, "{}", state)
    }
}
