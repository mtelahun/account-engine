use std::sync::Arc;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::xact_type::XactType;

use super::Account;

#[derive(Debug)]
pub struct JournalEntry {
    _account: Arc<Account>,
    _datetime: NaiveDateTime,
    _xact_type: XactType,
    _amount: Decimal,
}
