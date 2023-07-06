// pub mod domain_expression;
pub mod error;
pub mod general_ledger;
pub mod journal;
pub mod journal_transaction;
pub mod ledger;
pub mod ledger_line;
pub mod ledger_transaction;
pub mod ledger_xact_type;
pub mod period;
pub mod period_interim;

use async_trait::async_trait;

// Re-exports
pub use error::OrmError;
pub use general_ledger::GeneralLedgerService;
pub use journal::JournalService;
pub use journal_transaction::JournalTransactionService;
pub use ledger::LedgerService;
pub use period::AccountingPeriodService;

pub trait Resource {
    const NAME: &'static str;
}

#[async_trait]
pub trait Repository {
    async fn create_schema(&self) -> Result<(), OrmError>;
}

#[async_trait]
pub trait Period {
    async fn create_interim_periods();
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
