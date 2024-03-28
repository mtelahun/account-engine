use chrono::NaiveDateTime;

use crate::domain::subsidiary_ledger::account_id::AccountId;

use super::{ids::JournalId, TemplateColumnId};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountTransactionId(AccountId, NaiveDateTime);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournalTransactionId(JournalId, NaiveDateTime);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournalTransactionColumnId(JournalId, NaiveDateTime, TemplateColumnId);

impl AccountTransactionId {
    pub fn new(account_id: AccountId, dt: NaiveDateTime) -> Self {
        Self(account_id, dt)
    }

    pub fn account_id(&self) -> AccountId {
        self.0
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.1
    }
}

impl std::fmt::Display for AccountTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountTransactionId {{ Account ID: {}, Timestamp: {} }}",
            self.0, self.1
        )
    }
}

impl JournalTransactionId {
    pub fn new(jid: JournalId, dt: NaiveDateTime) -> Self {
        Self(jid, dt)
    }

    pub fn journal_id(&self) -> JournalId {
        self.0
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.1
    }
}

impl std::fmt::Display for JournalTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}",
            self.0, self.1
        )
    }
}

impl JournalTransactionColumnId {
    pub fn new(jid: JournalId, dt: NaiveDateTime, tcol_id: TemplateColumnId) -> Self {
        Self(jid, dt, tcol_id)
    }

    pub fn journal_id(&self) -> JournalId {
        self.0
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.1
    }

    pub fn template_column_id(&self) -> TemplateColumnId {
        self.2
    }
}

impl std::fmt::Display for JournalTransactionColumnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JournalTransactionColumnId {{ Journal ID: {}, Timestamp: {}, Template Column ID: {} }}",
            self.0, self.1, self.2
        )
    }
}
