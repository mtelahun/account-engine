use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::domain::{ids::JournalId, ArrayLongString, JournalTransactionId};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayLongString,
}

#[derive(Clone, Copy, Debug, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayLongString,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}
