use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    domain::{ids::JournalId, ColumnTotalId, JournalTransactionId, LedgerId},
    resource::TransactionState,
};

pub mod total;

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub sequence: usize,
    pub dr_ledger_id: Option<LedgerId>,
    pub cr_ledger_id: Option<LedgerId>,
    pub amount: Decimal,
    pub state: TransactionState,
    pub column_total_id: Option<ColumnTotalId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub sequence: usize,
    pub dr_ledger_id: Option<LedgerId>,
    pub cr_ledger_id: Option<LedgerId>,
    pub amount: Decimal,
    pub state: TransactionState,
    pub column_total_id: Option<ColumnTotalId>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            sequence: value.sequence,
            dr_ledger_id: value.dr_ledger_id,
            cr_ledger_id: value.cr_ledger_id,
            amount: value.amount,
            state: value.state,
            column_total_id: value.column_total_id,
        }
    }
}
