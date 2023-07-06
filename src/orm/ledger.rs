use async_trait::async_trait;

use crate::{
    domain::AccountId,
    entity::{
        journal_entry, ledger, ledger_intermediate, ledger_leaf, ledgers::account::ledger_derived,
        PostingRef,
    },
};

use super::{OrmError, Resource};

#[async_trait]
pub trait LedgerService {
    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError>;

    async fn journal_entries(
        &self,
        id: AccountId,
    ) -> Result<Vec<journal_entry::ActiveModel>, OrmError>;

    async fn journal_entry_by_posting_ref(
        &self,
        posting_ref: PostingRef,
    ) -> Result<Option<journal_entry::ActiveModel>, OrmError>;
}

impl Resource for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl Resource for ledger_derived::ActiveModel {
    const NAME: &'static str = "ledger_derived";
}

impl Resource for ledger_intermediate::ActiveModel {
    const NAME: &'static str = "ledger_intermediate";
}

impl Resource for ledger_leaf::ActiveModel {
    const NAME: &'static str = "ledger_leaf";
}
