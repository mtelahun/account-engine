use async_trait::async_trait;

use crate::{
    domain::{ids::InterimPeriodId, PeriodId},
    resource::{
        account_engine::AccountEngine,
        accounting_period::{self, interim_period},
    },
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
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
impl AccountingPeriodService<PostgresStore> for AccountEngine<PostgresStore> {
    fn repository(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl AccountingPeriodService<MemoryStore> for AccountEngine<MemoryStore> {
    fn repository(&self) -> &MemoryStore {
        &self.repository
    }
}
