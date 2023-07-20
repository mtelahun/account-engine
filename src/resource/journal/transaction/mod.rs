use postgres_types::{FromSql, ToSql};

use crate::{domain::AccountId, resource::LedgerKey};

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
    pub(crate) account_id: AccountId,
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

impl std::fmt::Display for PostingRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "PostingRef{{key: {}, account_id: {}}}",
            self.key, self.account_id
        );
        write!(f, "{msg}")
    }
}

use chrono::NaiveDateTime;

use crate::domain::{ids::JournalId, ArrayLongString, JournalTransactionId};

#[derive(Clone, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayLongString,
    pub lines: Vec<line::Model>,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayLongString,
    pub lines: Vec<line::ActiveModel>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}

pub mod line;
pub mod record;
