use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::{
    xact_type::XactType, AccountId, ExternalXactTypeCode, JournalTransactionId, LedgerXactTypeCode,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JournalEntry {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
    pub xact_type: XactType,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LedgerKey {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedgerEntry {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
    pub ledger_xact_type: LedgerXactType,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedgerTransaction {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
    pub ledger_dr: AccountId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalTransaction {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
    pub xact_type: XactType,
    pub external_xact_type: ExternalXactType,
    pub account_no: AccountId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedgerXactType {
    pub code: LedgerXactTypeCode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalXactType {
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub code: ExternalXactTypeCode,
}

#[derive(Clone, Debug)]
struct LedgerXactTypeDescription {
    pub _code: LedgerXactTypeCode,
    pub _description: String,
}

#[derive(Clone, Debug)]
pub struct ExternalXactTypeDescription {
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub code: ExternalXactTypeCode,
    pub description: String,
}
