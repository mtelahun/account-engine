use chrono::NaiveDateTime;

use crate::{
    domain::{
        ids::{AccountId, JournalId},
        ExternalXactTypeCode, JournalTransactionId, SpecJournalTemplateId,
    },
    resource::{PostingRef, TransactionState},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SpecJournalTemplateId,
    pub account_id: AccountId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SpecJournalTemplateId,
    pub account_id: AccountId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            template_id: value.template_id,
            account_id: value.account_id,
            xact_type_external: value.xact_type_external,
            state: value.state,
            posting_ref: value.posting_ref,
        }
    }
}
