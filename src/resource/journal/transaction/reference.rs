use crate::{
    domain::special_journal::column_total_id::ColumnTotalId,
    resource::journal::JournalType,
    shared_kernel::{JournalRefId, JournalTransactionId},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Model {
    pub journal_type: JournalType,
    pub journal_transaction: Option<JournalTransactionId>,
    pub column_total: Option<ColumnTotalId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    id: JournalRefId,
    pub journal_type: JournalType,
    pub journal_transaction: Option<JournalTransactionId>,
    pub column_total: Option<ColumnTotalId>,
}

impl ActiveModel {
    pub fn from_model(model: &Model) -> Self {
        Self {
            id: JournalRefId::new(),
            journal_type: model.journal_type,
            journal_transaction: model.journal_transaction,
            column_total: model.column_total,
        }
    }

    pub fn id(&self) -> JournalRefId {
        self.id
    }
}
