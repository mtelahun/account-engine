pub mod error;

// Re-exports
pub use error::OrmError;

use crate::entity::{
    accounting_period, general_ledger, interim_accounting_period, journal, journal_line, ledger,
};

pub trait RepositoryEntity {
    const NAME: &'static str;
}

pub trait AccountRepository<M, AM, I>
where
    AM: RepositoryEntity,
{
    fn create(&self, model: &M) -> Result<AM, OrmError>;

    fn search(&self, ids: Option<&[I]>) -> Vec<AM>;

    fn update(&self, ids: &[I], model: &M) -> Result<(), OrmError>;
}

impl RepositoryEntity for accounting_period::ActiveModel {
    const NAME: &'static str = "period";
}

impl RepositoryEntity for interim_accounting_period::ActiveModel {
    const NAME: &'static str = "period_interim";
}

impl RepositoryEntity for general_ledger::ActiveModel {
    const NAME: &'static str = "general_ledger";
}

impl RepositoryEntity for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl RepositoryEntity for journal::ActiveModel {
    const NAME: &'static str = "journal";
}

impl RepositoryEntity for journal_line::ActiveModel {
    const NAME: &'static str = "journal_xact";
}

// #[derive(Clone)]
// pub struct RepoConnection<M, AM, I>(Arc<dyn RepositoryOrm<M, AM, I>>);
