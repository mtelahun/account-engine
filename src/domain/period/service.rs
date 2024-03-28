use async_trait::async_trait;

use crate::{
    domain::ServiceError,
    infrastructure::data::db_context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period::{self, interim_period},
    },
    Store,
};

use super::{interim_period::InterimPeriodId, period_id::PeriodId};

#[async_trait]
pub trait AccountingPeriodService<R>
where
    R: Store
        + RepositoryOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + RepositoryOperations<interim_period::Model, interim_period::ActiveModel, InterimPeriodId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn get_interim_periods(
        &self,
        ids: Option<&Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_period::ActiveModel>, ServiceError> {
        Ok(<R as RepositoryOperations<
            interim_period::Model,
            interim_period::ActiveModel,
            InterimPeriodId,
        >>::get(self.store(), ids)
        .await?)
    }
}

#[async_trait]
impl AccountingPeriodService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl AccountingPeriodService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
