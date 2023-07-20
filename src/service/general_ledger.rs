use async_trait::async_trait;

use crate::{
    domain::{
        ids::{InterimPeriodId, JournalId},
        GeneralLedgerId, LedgerId, PeriodId,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period, general_ledger, journal,
        ledger::{self, LedgerType},
        InterimType,
    },
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Store,
};

use super::ServiceError;

#[async_trait]
pub trait GeneralLedgerService<R>
where
    R: Store
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<ledger::intermediate::Model, ledger::intermediate::ActiveModel, LedgerId>
        + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + ResourceOperations<
            accounting_period::interim_period::Model,
            accounting_period::interim_period::ActiveModel,
            InterimPeriodId,
        > + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn get_general_ledger(&self) -> Result<general_ledger::ActiveModel, ServiceError> {
        let gl: Vec<general_ledger::ActiveModel> = self.store().get(None).await?;

        Ok(gl[0])
    }

    async fn update_general_ledger(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, ServiceError> {
        let root: Vec<ledger::ActiveModel> = self.store().get(None).await?;
        let gl: Vec<general_ledger::ActiveModel> = self.store().get(None).await?;
        let mut gl = gl[0];
        gl.name = model.name;
        gl.currency_code = model.currency_code;
        gl.root = root[0].id;
        let _ = self.store().save(&gl).await?;

        Ok(gl)
    }

    async fn create_ledger(
        &self,
        model: &ledger::Model,
    ) -> Result<ledger::ActiveModel, ServiceError> {
        let parent: Vec<ledger::ActiveModel> = match model.parent_id {
            Some(id) => self.store().get(Some(&vec![id])).await?,
            None => return Err(ServiceError::Validation("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(ServiceError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        if self
            .store()
            .find_ledger_by_no(model.ledger_no)
            .await?
            .is_some()
        {
            return Err(ServiceError::Validation(format!(
                "duplicate ledger number: {}",
                model.ledger_no
            )));
        }
        let ledger = self.store().insert(model).await?;
        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger::intermediate::Model { id: ledger.id };
            let _ = self.store().insert(&intermediate).await?;
        } else {
            let account = ledger::leaf::Model { id: ledger.id };
            let _ = self.store().insert(&account).await?;
        }

        Ok(ledger)
    }

    async fn get_ledgers(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<ledger::ActiveModel>, ServiceError> {
        Ok(
            <R as ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>>::get(
                self.store(),
                ids,
            )
            .await?,
        )
    }

    async fn update_ledger(
        &self,
        _model: &ledger::Model,
    ) -> Result<ledger::ActiveModel, ServiceError> {
        todo!()
    }

    async fn create_journal(
        &self,
        model: &journal::Model,
    ) -> Result<journal::ActiveModel, ServiceError> {
        Ok(self.store().insert(model).await?)
    }

    async fn get_journals(
        &self,
        ids: Option<&Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, ServiceError> {
        Ok(
            <R as ResourceOperations<journal::Model, journal::ActiveModel, JournalId>>::get(
                self.store(),
                ids,
            )
            .await?,
        )
    }

    async fn update_journal(
        &self,
        _model: &journal::Model,
    ) -> Result<journal::ActiveModel, ServiceError> {
        todo!()
    }

    async fn create_period(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, ServiceError> {
        let period = self
            .store()
            .find_period_by_fiscal_year(model.fiscal_year)
            .await?;
        if period.is_some() {
            return Err(ServiceError::Validation(
                "duplicate accounting period".into(),
            ));
        }

        let active_model = self.store().insert(model).await?;
        let _ = match model.period_type {
            InterimType::CalendarMonth => active_model.create_interim_calendar(self.store()).await,
            InterimType::FourWeek => todo!(),
            InterimType::FourFourFiveWeek => todo!(),
        }
        .map_err(|s| {
            ServiceError::Unknown(format!(
                "failed to create interim periods for fiscal year {}: {s}",
                model.fiscal_year
            ))
        })?;

        Ok(active_model)
    }

    async fn get_periods(
        &self,
        ids: Option<&Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, ServiceError> {
        Ok(<R as ResourceOperations<
            accounting_period::Model,
            accounting_period::ActiveModel,
            PeriodId,
        >>::get(self.store(), ids)
        .await?)
    }
}

#[async_trait]
impl GeneralLedgerService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl GeneralLedgerService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
