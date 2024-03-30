use chrono::NaiveDateTime;

use crate::shared_kernel::ArrayString64;

use crate::domain::entity::{
    general_journal_transaction::journal_transaction_id::JournalTransactionId,
    journal::journal_id::JournalId,
    special_journal_template::special_journal_template_id::SpecialJournalTemplateId,
    subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
};

#[derive(Clone, Copy, Debug)]
pub struct JournalTransaction {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub explanation: ArrayString64,
}

#[derive(Clone, Copy, Debug)]
pub struct SpecialJournalTransaction {
    journal_id: JournalId,
    timestamp: NaiveDateTime,
    pub explanation: ArrayString64,
    template_id: SpecialJournalTemplateId,
    external_xact_type_code: ExternalXactTypeCode,
}

impl JournalTransaction {
    pub fn build_special_transaction(
        self,
        template_id: SpecialJournalTemplateId,
        external_xact_type_code: ExternalXactTypeCode,
    ) -> SpecialJournalTransaction {
        SpecialJournalTransaction {
            journal_id: self.journal_id,
            timestamp: self.timestamp,
            explanation: self.explanation,
            template_id,
            external_xact_type_code,
        }
    }

    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn journal_id(&self) -> JournalId {
        self.journal_id
    }

    pub fn new(journal_id: &JournalId, timestamp: NaiveDateTime, explanation: &str) -> Self {
        Self {
            journal_id: *journal_id,
            timestamp,
            explanation: explanation.into(),
        }
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
}

impl SpecialJournalTransaction {
    pub fn external_xact_type_code(&self) -> ExternalXactTypeCode {
        self.external_xact_type_code
    }

    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }

    pub fn journal_id(&self) -> JournalId {
        self.journal_id
    }

    pub fn template_id(&self) -> SpecialJournalTemplateId {
        self.template_id
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
}
