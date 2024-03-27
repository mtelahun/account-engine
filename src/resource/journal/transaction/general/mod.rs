use chrono::NaiveDateTime;

use crate::domain::{ids::JournalId, ArrayString128, JournalTransactionId};

pub mod line;

#[derive(Clone, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString128,
    pub lines: Vec<line::Model>,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString128,
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