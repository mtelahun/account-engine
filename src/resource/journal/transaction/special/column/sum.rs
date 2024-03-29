use rust_decimal::Decimal;

use crate::{
    domain::entity::{
        column_total::column_total_id::ColumnTotalId,
        general_journal_transaction::journal_transaction_id::JournalTransactionId,
    },
    resource::LedgerPostingRef,
    shared_kernel::Sequence,
};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub summary_id: JournalTransactionId,
    pub sequence: Sequence,
    pub amount: Decimal,
    pub posting_ref_dr: Option<LedgerPostingRef>,
    pub posting_ref_cr: Option<LedgerPostingRef>,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: ColumnTotalId,
    pub summary_id: JournalTransactionId,
    pub sequence: Sequence,
    pub amount: Decimal,
    pub posting_ref_dr: Option<LedgerPostingRef>,
    pub posting_ref_cr: Option<LedgerPostingRef>,
}

impl ActiveModel {
    pub fn from_model(value: &Model) -> Self {
        Self {
            id: ColumnTotalId::new(),
            summary_id: value.summary_id,
            sequence: value.sequence,
            amount: value.amount,
            posting_ref_dr: value.posting_ref_dr,
            posting_ref_cr: value.posting_ref_cr,
        }
    }
}
