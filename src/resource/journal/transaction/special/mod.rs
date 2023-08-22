use crate::domain::{
    ids::JournalId, AccountId, ArrayLongString, ExternalXactTypeCode, JournalTransactionId,
    SubJournalTemplateId,
};
use chrono::NaiveDateTime;

use super::{AccountPostingRef, TransactionState};

pub mod column;
pub mod template;
pub mod totals;

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SubJournalTemplateId,
    pub account_id: AccountId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub explanation: ArrayLongString,
    pub posting_ref: Option<AccountPostingRef>,
    pub account_posted_state: TransactionState,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SubJournalTemplateId,
    pub account_id: AccountId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub explanation: ArrayLongString,
    pub posting_ref: Option<AccountPostingRef>,
    pub account_posted_state: TransactionState,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            account_id: value.account_id,
            xact_type_external: value.xact_type_external,
            explanation: value.explanation,
            posting_ref: value.posting_ref,
            account_posted_state: value.account_posted_state,
            template_id: value.template_id,
        }
    }
}
