use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    domain::{ids::JournalId, xact_type::XactType, JournalTransactionId, LedgerId},
    resource::{PostingRef, TransactionState},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub ledger_id: LedgerId,
    pub xact_type: XactType,
    pub amount: Decimal,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub ledger_id: LedgerId,
    pub xact_type: XactType,
    pub amount: Decimal,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
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
            ledger_id: value.ledger_id,
            xact_type: value.xact_type,
            amount: value.amount,
            state: value.state,
            posting_ref: value.posting_ref,
        }
    }
}
