use std::sync::Arc;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::JournalTransactionId;

use super::{Account, Journal};

#[derive(Clone, Debug)]
pub struct JournalTransactionModel {
    pub timestamp: NaiveDateTime,
    pub journal: Arc<Journal>,
    pub posted: bool,
    pub amount: Decimal,
    pub acc_no_dr: String,
    pub acc_no_cr: String,
    pub description: String,
    pub posting_ref: Option<String>,
}

#[derive(Clone, Debug)]
pub struct JournalTransaction {
    pub id: JournalTransactionId,
    pub timestamp: NaiveDateTime,
    pub journal: Arc<Journal>,
    pub posted: bool,
    pub amount: Decimal,
    pub account_dr: Arc<Account>,
    pub account_cr: Arc<Account>,
    pub description: String,
    pub posting_ref: Option<String>,
}
