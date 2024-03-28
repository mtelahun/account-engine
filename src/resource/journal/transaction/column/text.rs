use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::shared_kernel::{
    composite_ids::JournalTransactionColumnId, ArrayString128, JournalId, TemplateColumnId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub value: ArrayString128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub value: ArrayString128,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            value: value.value,
        }
    }
}
