use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    entity::{
        journal_transaction, journal_transaction_line_account, journal_transaction_line_ledger,
        journal_transaction_record,
    },
};

use super::{OrmError, Resource};

#[async_trait]
pub trait JournalTransactionService {
    async fn create(
        &self,
        model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError>;

    async fn retrieve(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction::ActiveModel>, OrmError>;
}

// The journal_transaction::ActiveModel is only ever used to communicate with
// the caller and doesn't have any datastore models associated with it.
impl Resource for journal_transaction::ActiveModel {
    const NAME: &'static str = "";
}

impl Resource for journal_transaction_record::ActiveModel {
    const NAME: &'static str = "journal_transaction_record";
}

impl Resource for journal_transaction_line_ledger::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_ledger";
}

impl Resource for journal_transaction_line_account::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_account";
}
