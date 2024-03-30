use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::{
    domain::entity::{
        journal::journal_id::JournalId,
        journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
        special_journal_template_column::template_column_id::TemplateColumnId,
    },
    shared_kernel::ArrayString64,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ColumnText {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub value: ArrayString64,
}

impl ColumnText {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }
}
