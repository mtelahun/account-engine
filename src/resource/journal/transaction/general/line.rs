use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    domain::entity::ledger::ledger_id::LedgerId,
    resource::{LedgerPostingRef, TransactionState},
    shared_kernel::{JournalId, JournalTransactionId},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub dr_ledger_id: LedgerId,
    pub cr_ledger_id: LedgerId,
    pub amount: Decimal,
    pub state: TransactionState,
    pub dr_posting_ref: Option<LedgerPostingRef>,
    pub cr_posting_ref: Option<LedgerPostingRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub dr_ledger_id: LedgerId,
    pub cr_ledger_id: LedgerId,
    pub amount: Decimal,
    pub state: TransactionState,
    pub dr_posting_ref: Option<LedgerPostingRef>,
    pub cr_posting_ref: Option<LedgerPostingRef>,
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
            dr_ledger_id: value.dr_ledger_id,
            cr_ledger_id: value.cr_ledger_id,
            amount: value.amount,
            state: value.state,
            dr_posting_ref: value.dr_posting_ref,
            cr_posting_ref: value.cr_posting_ref,
        }
    }
}
