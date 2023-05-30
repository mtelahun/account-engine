use std::sync::Arc;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::{AccountId, JournalTransactionId};

use super::{
    ledger_entry::{LedgerKey, LedgerXactType},
    Journal,
};

#[derive(Clone, Debug)]
pub struct JournalTransactionModel {
    pub timestamp: NaiveDateTime,
    pub journal: Arc<Journal>,
    pub state: TransactionState,
    pub amount: Decimal,
    pub acc_no_dr: String,
    pub acc_no_cr: String,
    pub description: String,
    pub posting_ref: Option<PostingRef>,
}

#[derive(Clone, Debug)]
pub struct JournalTransaction {
    pub id: JournalTransactionId,
    pub timestamp: NaiveDateTime,
    pub journal: Arc<Journal>,
    pub state: TransactionState,
    pub amount: Decimal,
    pub account_dr: AccountId,
    pub account_cr: AccountId,
    pub description: String,
    pub posting_ref: Option<PostingRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionState {
    Pending,
    Archived,
    Posted,
}

#[derive(Clone, Copy, Debug)]
pub struct PostingRef(LedgerKey, LedgerXactType);

impl PostingRef {
    pub fn new(key_dr: LedgerKey, ledger_xact_type: LedgerXactType) -> Self {
        Self(key_dr, ledger_xact_type)
    }

    pub fn ledger_key(&self) -> LedgerKey {
        self.0
    }

    pub fn xact_type(&self) -> LedgerXactType {
        self.1
    }
}
