use rust_decimal::Decimal;

use crate::{
    domain::{ColumnTotalId, JournalTransactionId},
    resource::LedgerPostingRef,
};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub transaction_id: JournalTransactionId,
    pub sequence: usize,
    pub total: Decimal,
    pub posting_ref_dr: Option<LedgerPostingRef>,
    pub posting_ref_cr: Option<LedgerPostingRef>,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: ColumnTotalId,
    pub transaction_id: JournalTransactionId,
    pub sequence: usize,
    pub total: Decimal,
    pub posting_ref_dr: Option<LedgerPostingRef>,
    pub posting_ref_cr: Option<LedgerPostingRef>,
}

impl ActiveModel {
    pub fn from_model(value: &Model) -> Self {
        Self {
            id: ColumnTotalId::new(),
            transaction_id: value.transaction_id,
            sequence: value.sequence,
            total: value.total,
            posting_ref_dr: value.posting_ref_dr,
            posting_ref_cr: value.posting_ref_cr,
        }
    }
}
