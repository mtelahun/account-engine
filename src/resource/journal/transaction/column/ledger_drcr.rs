use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::domain::entity::{
    column_total::column_total_id::ColumnTotalId, general_journal::journal_id::JournalId,
    journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
    ledger::ledger_id::LedgerId,
    special_journal_template_column::template_column_id::TemplateColumnId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub amount: Decimal,
    pub ledger_dr_id: LedgerId,
    pub ledger_cr_id: LedgerId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub amount: Decimal,
    pub ledger_dr_id: LedgerId,
    pub ledger_cr_id: LedgerId,
    pub column_total_id: Option<ColumnTotalId>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}

impl From<Model> for ActiveModel {
    fn from(value: Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            amount: value.amount,
            ledger_cr_id: value.ledger_cr_id,
            ledger_dr_id: value.ledger_dr_id,
            column_total_id: None,
        }
    }
}
