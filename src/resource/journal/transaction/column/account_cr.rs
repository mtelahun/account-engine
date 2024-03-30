use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::{
    domain::entity::{
        external_account::account_id::AccountId,
        journal::journal_id::JournalId,
        journal_transaction_column::{
            account_cr::ColumnAccountCr, journal_transaction_column_id::JournalTransactionColumnId,
        },
        special_journal_template_column::template_column_id::TemplateColumnId,
    },
    resource::journal::transaction::AccountPostingRef,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub account_id: AccountId,
    pub amount: Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub account_id: AccountId,
    pub amount: Decimal,
    pub posting_ref: Option<AccountPostingRef>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }

    pub fn posted(&self) -> bool {
        self.posting_ref.is_some()
    }
}

impl From<Model> for ActiveModel {
    fn from(value: Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            amount: value.amount,
            account_id: value.account_id,
            posting_ref: None,
        }
    }
}

impl From<ActiveModel> for ColumnAccountCr {
    fn from(value: ActiveModel) -> Self {
        ColumnAccountCr {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            account_id: value.account_id,
            amount: value.amount,
            posting_ref: value.posting_ref,
        }
    }
}

impl From<ColumnAccountCr> for ActiveModel {
    fn from(value: ColumnAccountCr) -> Self {
        ActiveModel {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_column_id: value.template_column_id,
            account_id: value.account_id,
            amount: value.amount,
            posting_ref: value.posting_ref,
        }
    }
}
