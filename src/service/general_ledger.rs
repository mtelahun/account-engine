use async_trait::async_trait;

use crate::{
    domain::{
        ids::{InterimPeriodId, JournalId},
        AccountId, GeneralLedgerId, PeriodId,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period, general_ledger, journal,
        ledger::{self, LedgerType},
        InterimType,
    },
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Repository,
};

use super::ServiceError;

#[async_trait]
pub trait GeneralLedgerService<R>
where
    R: Repository
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
        + ResourceOperations<
            ledger::intermediate::Model,
            ledger::intermediate::ActiveModel,
            AccountId,
        > + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, AccountId>
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
    fn repository(&self) -> &R;

    async fn get_general_ledger(&self) -> Result<general_ledger::ActiveModel, ServiceError> {
        let gl: Vec<general_ledger::ActiveModel> = self.repository().get(None).await?;

        Ok(gl[0])
    }

    async fn update_general_ledger(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, ServiceError> {
        let root: Vec<ledger::ActiveModel> = self.repository().get(None).await?;
        let gl: Vec<general_ledger::ActiveModel> = self.repository().get(None).await?;
        let mut gl = gl[0];
        gl.name = model.name;
        gl.currency_code = model.currency_code;
        gl.root = root[0].id;
        let _ = self.repository().save(&gl).await?;

        Ok(gl)
    }

    async fn create_ledger(
        &self,
        model: &ledger::Model,
    ) -> Result<ledger::ActiveModel, ServiceError> {
        let parent: Vec<ledger::ActiveModel> = match model.parent_id {
            Some(id) => self.repository().get(Some(&vec![id])).await?,
            None => return Err(ServiceError::Validation("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(ServiceError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        if self
            .repository()
            .find_ledger_by_no(model.ledger_no)
            .await?
            .is_some()
        {
            return Err(ServiceError::Validation(format!(
                "duplicate ledger number: {}",
                model.ledger_no
            )));
        }
        let ledger = self.repository().insert(model).await?;
        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger::intermediate::Model { id: ledger.id };
            let _ = self.repository().insert(&intermediate).await?;
        } else {
            let account = ledger::leaf::Model { id: ledger.id };
            let _ = self.repository().insert(&account).await?;
        }

        Ok(ledger)
    }

    async fn get_ledgers(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, ServiceError> {
        Ok(
            <R as ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>>::get(
                self.repository(),
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
        let id = JournalId::new();
        let journal = journal::ActiveModel {
            id,
            name: model.name,
            code: model.code,
        };
        self.repository().insert(model).await?;

        Ok(journal)
    }

    async fn get_journals(
        &self,
        ids: Option<&Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, ServiceError> {
        Ok(
            <R as ResourceOperations<journal::Model, journal::ActiveModel, JournalId>>::get(
                self.repository(),
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
            .repository()
            .find_period_by_fiscal_year(model.fiscal_year)
            .await?;
        if period.is_some() {
            return Err(ServiceError::Validation(
                "duplicate accounting period".into(),
            ));
        }

        let active_model = self.repository().insert(model).await?;
        let _ = match model.period_type {
            InterimType::CalendarMonth => {
                active_model
                    .create_interim_calendar(self.repository())
                    .await
            }
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
        >>::get(self.repository(), ids)
        .await?)
    }
}

#[async_trait]
impl GeneralLedgerService<PostgresStore> for AccountEngine<PostgresStore> {
    fn repository(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl GeneralLedgerService<MemoryStore> for AccountEngine<MemoryStore> {
    fn repository(&self) -> &MemoryStore {
        &self.repository
    }
}