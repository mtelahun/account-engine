use async_trait::async_trait;

use crate::{
    domain::{ids::InterimPeriodId, PeriodId},
    entity::{
        account_engine::AccountEngine,
        accounting_period::{self, interim_period},
        InterimType,
    },
    resource::{postgres::repository::PostgresRepository, OrmError, ResourceOperations},
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

    async fn create(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError>;

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

    async fn create(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let periods = self.repository.find_period_by_year(model).await?;
        if periods.is_empty() {
            let active_model = self.repository.insert(model).await?;

            let _ = match model.period_type {
                InterimType::CalendarMonth => {
                    active_model.create_interim_calendar(&self.repository).await
                }
                InterimType::FourWeek => todo!(),
                InterimType::FourFourFiveWeek => todo!(),
            }
            .map_err(OrmError::Internal)?;

            return Ok(active_model);
        }

        Err(OrmError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }
}
