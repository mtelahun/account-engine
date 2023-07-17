use async_trait::async_trait;

pub mod error;
pub mod memory;
pub mod postgres;

pub use error::OrmError;

use crate::resource::{accounting_period, general_ledger, journal, ledger, ledger_xact_type};

pub trait Resource {
    const NAME: &'static str;
}

#[async_trait]
pub trait ResourceOperations<M, AM, I>
where
    M: Send + Sync,
    AM: Resource + Send + Sync,
    I: Send + Sync,
{
    async fn insert(&self, model: &M) -> Result<AM, OrmError>;

    async fn get(&self, ids: Option<&Vec<I>>) -> Result<Vec<AM>, OrmError>;

    async fn search(&self, domain: &str) -> Result<Vec<AM>, OrmError>;

    async fn save(&self, model: &AM) -> Result<u64, OrmError>;

    async fn delete(&self, id: I) -> Result<u64, OrmError>;

    async fn archive(&self, id: I) -> Result<u64, OrmError>;

    async fn unarchive(&self, id: I) -> Result<u64, OrmError>;
}

impl Resource for general_ledger::ActiveModel {
    const NAME: &'static str = "general_ledger";
}

impl Resource for accounting_period::interim_period::ActiveModel {
    const NAME: &'static str = "interim_accounting_period";
}

impl Resource for journal::transaction::line::account::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_account";
}

impl Resource for journal::transaction::line::ledger::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_ledger";
}

impl Resource for journal::transaction::record::ActiveModel {
    const NAME: &'static str = "journal_transaction_record";
}

/// The journal_transaction::ActiveModel is only ever used to communicate with
/// the caller and doesn't have any datastore models associated with it.
impl Resource for journal::transaction::ActiveModel {
    const NAME: &'static str = "";
}

impl Resource for journal::ActiveModel {
    const NAME: &'static str = "journal";
}

impl Resource for ledger::intermediate::ActiveModel {
    const NAME: &'static str = "ledger_intermediate";
}

impl Resource for ledger::leaf::ActiveModel {
    const NAME: &'static str = "ledger_leaf";
}

impl Resource for ledger::transaction::ActiveModel {
    const NAME: &'static str = "ledger_transaction";
}

impl Resource for ledger::transaction::ledger::ActiveModel {
    const NAME: &'static str = "ledger_transaction_ledger";
}

impl Resource for ledger_xact_type::ActiveModel {
    const NAME: &'static str = "ledger_transaction_type";
}

impl Resource for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl Resource for ledger::derived::ActiveModel {
    const NAME: &'static str = "ledger_derived";
}

impl Resource for accounting_period::ActiveModel {
    const NAME: &'static str = "accounting_period";
}
