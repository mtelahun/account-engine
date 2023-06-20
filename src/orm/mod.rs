pub mod error;

use async_trait::async_trait;
// Re-exports
pub use error::OrmError;

use crate::entity::{
    accounting_period, general_ledger, interim_accounting_period, journal, journal_transaction,
    journal_transaction_line_ledger, journal_transaction_record,
    jrnl::transaction::journal_transaction_line_account, ledger, ledger_intermediate, ledger_leaf,
    ledger_line, ledger_transaction, ledger_xact_type, ledgers::account::ledger_derived,
};

pub trait RepositoryEntity {
    const NAME: &'static str;
}

#[async_trait]
pub trait AccountRepository<M, AM, I>
where
    M: Send + Sync,
    AM: RepositoryEntity + Send + Sync,
    I: Send + Sync,
{
    async fn create(&self, model: &M) -> Result<AM, OrmError>;

    async fn search(&self, ids: Option<Vec<I>>) -> Result<Vec<AM>, OrmError>;

    async fn update(&self, ids: &[I], active_model: &AM) -> Result<u64, OrmError>;
}

impl RepositoryEntity for accounting_period::ActiveModel {
    const NAME: &'static str = "accounting_period";
}

impl RepositoryEntity for interim_accounting_period::ActiveModel {
    const NAME: &'static str = "interim_accounting_period";
}

impl RepositoryEntity for general_ledger::ActiveModel {
    const NAME: &'static str = "general_ledger";
}

impl RepositoryEntity for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl RepositoryEntity for ledger_intermediate::ActiveModel {
    const NAME: &'static str = "ledger_intermediate";
}

impl RepositoryEntity for ledger_leaf::ActiveModel {
    const NAME: &'static str = "ledger_leaf";
}

impl RepositoryEntity for ledger_derived::ActiveModel {
    const NAME: &'static str = "ledger_derived";
}

impl RepositoryEntity for journal::ActiveModel {
    const NAME: &'static str = "journal";
}

impl RepositoryEntity for journal_transaction_record::ActiveModel {
    const NAME: &'static str = "journal_transaction_record";
}

impl RepositoryEntity for journal_transaction_line_ledger::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_ledger";
}

impl RepositoryEntity for journal_transaction_line_account::ActiveModel {
    const NAME: &'static str = "journal_transaction_line_account";
}

// The journal_transaction::ActiveModel is only ever used to communicate with
// the caller and doesn't have any datastore models associated with it.
impl RepositoryEntity for journal_transaction::ActiveModel {
    const NAME: &'static str = "";
}

impl RepositoryEntity for ledger_line::ActiveModel {
    const NAME: &'static str = "ledger_line";
}

impl RepositoryEntity for ledger_transaction::ActiveModel {
    const NAME: &'static str = "ledger_transaction";
}

impl RepositoryEntity for ledger_xact_type::ActiveModel {
    const NAME: &'static str = "ledger_transaction_type";
}

// #[derive(Clone)]
// pub struct RepoConnection<M, AM, I>(Arc<dyn RepositoryOrm<M, AM, I>>);
