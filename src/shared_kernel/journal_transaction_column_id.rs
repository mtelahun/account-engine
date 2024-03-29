use chrono::NaiveDateTime;

use crate::domain::special_journal::template_column_id::TemplateColumnId;

use super::JournalId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournalTransactionColumnId(JournalId, NaiveDateTime, TemplateColumnId);

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
