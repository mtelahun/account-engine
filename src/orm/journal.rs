use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    entity::{journal, journal_transaction},
};

use super::{OrmError, Resource};

#[async_trait]
pub trait JournalService {
    async fn create(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError>;

    async fn add_transaction(
        &self,
        model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError>;

    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, OrmError>;
}

impl Resource for journal::ActiveModel {
    const NAME: &'static str = "journal";
}
