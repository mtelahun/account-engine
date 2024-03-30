use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::domain::entity::{
    column_total::column_total_id::ColumnTotalId, journal::journal_id::JournalId,
    journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
    ledger::ledger_id::LedgerId,
    special_journal_template_column::template_column_id::TemplateColumnId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ColumnLedgerDrCr {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub amount: Decimal,
    pub ledger_dr_id: LedgerId,
    pub ledger_cr_id: LedgerId,
    pub column_total_id: Option<ColumnTotalId>,
}

impl ColumnLedgerDrCr {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}
