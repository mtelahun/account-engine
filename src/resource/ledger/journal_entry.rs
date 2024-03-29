use std::str::FromStr;

use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::{
    domain::entity::{
        general_journal_transaction::journal_transaction_id::JournalTransactionId,
        ledger::ledger_id::LedgerId,
        ledger_xact_type_code::{LedgerXactTypeCode, XACT_LEDGER},
        xact_type::XactType,
    },
    resource::ledger,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct LedgerKey {
    pub ledger_id: LedgerId,
    pub timestamp: NaiveDateTime,
}

impl std::fmt::Display for LedgerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "LedgerKey(id: {}, timestamp: {})",
            self.ledger_id, self.timestamp
        );
        write!(f, "{msg}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Model {
    pub ledger_id: LedgerId,
    pub timestamp: NaiveDateTime,
    pub xact_type: XactType,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActiveModel {
    pub ledger_id: LedgerId,
    pub timestamp: NaiveDateTime,
    pub xact_type: XactType,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

impl From<ledger::transaction::ActiveModel> for ActiveModel {
    fn from(value: ledger::transaction::ActiveModel) -> Self {
        let ll_code = LedgerXactTypeCode::from_str(XACT_LEDGER).unwrap();
        let xact_type = if value.ledger_xact_type_code == ll_code {
            XactType::Cr
        } else {
            XactType::Dr
        };
        Self {
            xact_type,
            ledger_id: value.ledger_id,
            timestamp: value.timestamp,
            amount: value.amount,
            journal_ref: value.journal_ref,
        }
    }
}
