use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::{
    domain::{
        entity::external_account::account_id::AccountId,
        special_journal::template_column_id::TemplateColumnId,
    },
    resource::journal::transaction::AccountPostingRef,
    shared_kernel::{journal_transaction_column_id::JournalTransactionColumnId, JournalId},
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
