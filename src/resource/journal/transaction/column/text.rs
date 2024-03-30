use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::{
    domain::entity::{
        journal::journal_id::JournalId,
        journal_transaction_column::{
            journal_transaction_column_id::JournalTransactionColumnId, text::ColumnText,
        },
        special_journal_template_column::template_column_id::TemplateColumnId,
    },
    shared_kernel::ArrayString64,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub value: ArrayString64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub value: ArrayString64,
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

impl From<ActiveModel> for ColumnText {
    fn from(value: ActiveModel) -> Self {
        ColumnText {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            value: value.value,
        }
    }
}
