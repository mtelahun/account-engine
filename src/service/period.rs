use async_trait::async_trait;

use crate::{
    domain::{ids::InterimPeriodId, PeriodId},
    repository::{
        memory_store::repository::MemoryRepository, postgres::repository::PostgresRepository,
        ResourceOperations,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period::{self, interim_period},
    },
    Repository,
};

use super::ServiceError;

#[async_trait]
pub trait AccountingPeriodService<R>
where
    R: Repository
        + ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + ResourceOperations<interim_period::Model, interim_period::ActiveModel, InterimPeriodId>
        + Send
        + Sync
        + 'static,
{
    fn repository(&self) -> &R;

    async fn get_interim_periods(
        &self,
        ids: Option<&Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_period::ActiveModel>, ServiceError> {
        Ok(<R as ResourceOperations<
            interim_period::Model,
            interim_period::ActiveModel,
            InterimPeriodId,
        >>::get(self.repository(), ids)
        .await?)
    }
}

#[async_trait]
impl AccountingPeriodService<PostgresRepository> for AccountEngine<PostgresRepository> {
    fn repository(&self) -> &PostgresRepository {
        &self.repository
    }
}

#[async_trait]
impl AccountingPeriodService<MemoryRepository> for AccountEngine<MemoryRepository> {
    fn repository(&self) -> &MemoryRepository {
        &self.repository
    }
}
