use chrono::NaiveDateTime;

use crate::domain::entity::{
    general_journal::journal_id::JournalId,
    general_journal_transaction::journal_transaction_id::JournalTransactionId,
};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
}

impl ActiveModel {
    pub fn from_model(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
        }
    }

    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
}
