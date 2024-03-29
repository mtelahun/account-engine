use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    resource::journal::{
        self,
        transaction::{AccountPostingRef, JournalTransactionColumnType},
    },
    shared_kernel::{
        journal_transaction_column_id::JournalTransactionColumnId, ArrayString128, JournalId,
        JournalTransactionId,
    },
};

use super::{
    general_ledger::ledger_id::LedgerId,
    special_journal::{
        column_total_id::ColumnTotalId, special_journal_template_id::SpecialJournalTemplateId,
    },
    subsidiary_ledger::{account_id::AccountId, external_xact_type_code::ExternalXactTypeCode},
};

pub mod column;
pub mod general;
pub mod special;

pub trait JournalTransactionOps {}

#[derive(Copy, Clone, Debug)]
pub struct SpecialJournalTransaction<S: JournalTransactionOps> {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString128,
    pub(crate) extra: S,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalTransactionColumn {
    LedgerDrCr(journal::transaction::column::ledger_drcr::ActiveModel),
    Text(journal::transaction::column::text::ActiveModel),
    AccountDr(journal::transaction::column::account_dr::ActiveModel),
    AccountCr(journal::transaction::column::account_cr::ActiveModel),
}

impl JournalTransactionOps for journal::transaction::special::ActiveModel {}

impl<S: JournalTransactionOps> SpecialJournalTransaction<S> {
    pub fn new(base: &journal::transaction::ActiveModel, s: S) -> SpecialJournalTransaction<S> {
        SpecialJournalTransaction {
            journal_id: base.journal_id,
            timestamp: base.timestamp,
            explanation: base.explanation,
            extra: s,
        }
    }
}

impl SpecialJournalTransaction<journal::transaction::special::ActiveModel> {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
    pub fn template_id(&self) -> SpecialJournalTemplateId {
        self.extra.template_id
    }

    pub fn xact_type_external_code(&self) -> ExternalXactTypeCode {
        self.extra.xact_type_external.unwrap()
    }
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
