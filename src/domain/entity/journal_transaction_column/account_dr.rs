use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;

use crate::domain::entity::{
    account_posting_ref::AccountPostingRef, external_account::account_id::AccountId,
    journal::journal_id::JournalId,
    journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
    special_journal_template_column::template_column_id::TemplateColumnId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct ColumnAccountDr {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_column_id: TemplateColumnId,
    pub account_id: AccountId,
    pub amount: Decimal,
    pub posting_ref: Option<AccountPostingRef>,
}

impl ColumnAccountDr {
    pub fn id(&self) -> JournalTransactionColumnId {
        JournalTransactionColumnId::new(self.journal_id, self.timestamp, self.template_column_id)
    }

    pub fn posted(&self) -> bool {
        self.posting_ref.is_some()
    }
}
