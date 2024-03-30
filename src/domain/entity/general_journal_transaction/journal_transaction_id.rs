use chrono::NaiveDateTime;

use crate::domain::entity::journal::journal_id::JournalId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournalTransactionId(JournalId, NaiveDateTime);

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
