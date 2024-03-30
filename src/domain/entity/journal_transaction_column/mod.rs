use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::entity::{
    account_posting_ref::AccountPostingRef,
    column_total::column_total_id::ColumnTotalId,
    external_account::account_id::AccountId,
    journal::journal_id::JournalId,
    journal_transaction_column::{
        column_type::JournalTransactionColumnType,
        journal_transaction_column_id::JournalTransactionColumnId,
    },
    ledger::ledger_id::LedgerId,
};

pub mod account_cr;
pub mod account_dr;
pub mod column_type;
pub mod journal_transaction_column_id;
pub mod ledger_drcr;
pub mod text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalTransactionColumn {
    LedgerDrCr(ledger_drcr::ColumnLedgerDrCr),
    Text(text::ColumnText),
    AccountDr(account_dr::ColumnAccountDr),
    AccountCr(account_cr::ColumnAccountCr),
}

impl JournalTransactionColumn {
    pub fn id(&self) -> JournalTransactionColumnId {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.id(),
            JournalTransactionColumn::Text(c) => c.id(),
            JournalTransactionColumn::AccountDr(c) => c.id(),
            JournalTransactionColumn::AccountCr(c) => c.id(),
        }
    }

    pub fn journal_id(&self) -> JournalId {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.journal_id,
            JournalTransactionColumn::Text(c) => c.journal_id,
            JournalTransactionColumn::AccountDr(c) => c.journal_id,
            JournalTransactionColumn::AccountCr(c) => c.journal_id,
        }
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.timestamp,
            JournalTransactionColumn::Text(c) => c.timestamp,
            JournalTransactionColumn::AccountDr(c) => c.timestamp,
            JournalTransactionColumn::AccountCr(c) => c.timestamp,
        }
    }

    pub fn column_type(&self) -> JournalTransactionColumnType {
        match self {
            JournalTransactionColumn::LedgerDrCr(_) => JournalTransactionColumnType::LedgerDrCr,
            JournalTransactionColumn::Text(_) => JournalTransactionColumnType::Text,
            JournalTransactionColumn::AccountDr(_) => JournalTransactionColumnType::AccountDr,
            JournalTransactionColumn::AccountCr(_) => JournalTransactionColumnType::AccountCr,
        }
    }

    pub fn amount(&self) -> Decimal {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.amount,
            JournalTransactionColumn::Text(_) => Decimal::ZERO,
            JournalTransactionColumn::AccountDr(c) => c.amount,
            JournalTransactionColumn::AccountCr(c) => c.amount,
        }
    }

    pub fn posted(&self) -> bool {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.posted(),
            JournalTransactionColumn::Text(_) => false,
            JournalTransactionColumn::AccountDr(c) => c.posted(),
            JournalTransactionColumn::AccountCr(c) => c.posted(),
        }
    }

    pub fn ledger_dr_id(&self) -> Option<LedgerId> {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => Some(c.ledger_dr_id),
            _ => None,
        }
    }

    pub fn ledger_cr_id(&self) -> Option<LedgerId> {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => Some(c.ledger_cr_id),
            _ => None,
        }
    }

    pub fn account_id(&self) -> Option<AccountId> {
        match self {
            JournalTransactionColumn::AccountDr(c) => Some(c.account_id),
            JournalTransactionColumn::AccountCr(c) => Some(c.account_id),
            _ => None,
        }
    }

    pub fn column_total_id(&self) -> Option<ColumnTotalId> {
        match self {
            JournalTransactionColumn::LedgerDrCr(c) => c.column_total_id,
            _ => None,
        }
    }

    pub fn account_posting_ref(&self) -> Option<AccountPostingRef> {
        match self {
            JournalTransactionColumn::AccountDr(c) => c.posting_ref,
            JournalTransactionColumn::AccountCr(c) => c.posting_ref,
            _ => None,
        }
    }
}

impl From<account_cr::ColumnAccountCr> for JournalTransactionColumn {
    fn from(value: account_cr::ColumnAccountCr) -> Self {
        JournalTransactionColumn::AccountCr(value)
    }
}

impl From<account_dr::ColumnAccountDr> for JournalTransactionColumn {
    fn from(value: account_dr::ColumnAccountDr) -> Self {
        JournalTransactionColumn::AccountDr(value)
    }
}

impl From<ledger_drcr::ColumnLedgerDrCr> for JournalTransactionColumn {
    fn from(value: ledger_drcr::ColumnLedgerDrCr) -> Self {
        JournalTransactionColumn::LedgerDrCr(value)
    }
}

impl From<text::ColumnText> for JournalTransactionColumn {
    fn from(value: text::ColumnText) -> Self {
        JournalTransactionColumn::Text(value)
    }
}
