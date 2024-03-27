use crate::domain::{
    ids::JournalId, ArrayString128, ExternalXactTypeCode, JournalTransactionId,
    SpecialJournalTemplateId,
};
use chrono::NaiveDateTime;

use super::TransactionState;

pub mod column;
pub mod summary;
pub mod template;

#[derive(Clone, Copy, Debug, Default)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SpecialJournalTemplateId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SpecialJournalTemplateId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
}

#[derive(Clone, Copy, Debug)]
pub struct JournalTransactionSpecial {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub template_id: SpecialJournalTemplateId,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub explanation: ArrayString128,
    pub state: TransactionState,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn posted(&self) -> bool {
        todo!()
    }
}

impl JournalTransactionSpecial {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            xact_type_external: value.xact_type_external,
            template_id: value.template_id,
        }
    }
}
