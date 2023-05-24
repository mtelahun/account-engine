use std::sync::Arc;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::sequence::JournalSequence;

use super::{Account, Journal};

#[derive(Clone, Debug)]
pub struct JournalTransaction {
    pub journal: Arc<Journal>,
    pub sequence: JournalSequence,
    pub timestamp: NaiveDateTime,
    pub posted: bool,
    pub amount: Decimal,
    pub account_dr: Arc<Account>,
    pub account_cr: Arc<Account>,
    pub description: String,
    pub posting_ref: String,
}
