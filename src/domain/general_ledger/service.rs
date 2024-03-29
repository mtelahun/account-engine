use async_trait::async_trait;

use crate::{
    domain::{
        period::{interim_period::InterimPeriodId, period_id::PeriodId},
        Ledger, LedgerAccount, ServiceError,
    },
    infrastructure::data::db_context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period, general_ledger, journal,
        ledger::{self, LedgerType},
        InterimType,
    },
    shared_kernel::{ArrayString128, ArrayString24, ArrayString3, JournalId},
    Store,
};

use super::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId};

#[async_trait]
pub trait GeneralLedgerService<R>
where
    R: Store
        + RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<ledger::derived::Model, ledger::derived::ActiveModel, LedgerId>
        + RepositoryOperations<
            ledger::intermediate::Model,
            ledger::intermediate::ActiveModel,
            LedgerId,
        > + RepositoryOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
        + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + RepositoryOperations<
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
        ledger_type: LedgerType,
        parent_id: LedgerId,
        name: &str,
        number: &str,
        currency_code: Option<ArrayString3>,
    ) -> Result<LedgerAccount, ServiceError> {
        let parent: Vec<ledger::ActiveModel> = self.store().get(Some(&vec![parent_id])).await?;
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(ServiceError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }
        let base = ledger::Model {
            name: ArrayString128::from(name),
            number: ArrayString24::from(number),
            ledger_type,
            parent_id: Some(parent_id),
            currency_code,
        };
        let base = self.store().insert(&base).await?;
        let ledger = match ledger_type {
            LedgerType::Derived => {
                let model = ledger::derived::Model { id: base.id };
                let derived = self.store().insert(&model).await?;
                let derived = Ledger::new(base, derived);
                LedgerAccount::Derived(derived)
            }
            LedgerType::Intermediate => {
                let model = ledger::intermediate::Model { id: base.id };
                let intermediate = self.store().insert(&model).await?;
                let intermediate = Ledger::new(base, intermediate);
                LedgerAccount::Intermediate(intermediate)
            }
            LedgerType::Leaf => {
                let model = ledger::leaf::Model { id: base.id };
                let leaf = self.store().insert(&model).await?;
                let leaf = Ledger::new(base, leaf);
                LedgerAccount::Leaf(leaf)
            }
        };

        Ok(ledger)
    }

    async fn get_ledgers(
        &self,
        ids: Option<&Vec<LedgerId>>,
    ) -> Result<Vec<LedgerAccount>, ServiceError> {
        let ledgers =
            <R as RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>>::get(
                self.store(),
                ids,
            )
            .await?;

        let mut res = Vec::new();
        for ledger in ledgers {
            match ledger.ledger_type {
                LedgerType::Derived => {
                    let rows = <R as RepositoryOperations<
                        ledger::derived::Model,
                        ledger::derived::ActiveModel,
                        LedgerId,
                    >>::get(self.store(), Some(&vec![ledger.id]))
                    .await?;
                    res.push(LedgerAccount::Derived(
                        Ledger::<ledger::derived::ActiveModel>::new(ledger, rows[0]),
                    ))
                }
                LedgerType::Intermediate => {
                    let rows = <R as RepositoryOperations<
                        ledger::intermediate::Model,
                        ledger::intermediate::ActiveModel,
                        LedgerId,
                    >>::get(self.store(), Some(&vec![ledger.id]))
                    .await?;
                    eprintln!("id: {}, rows: {:?}", ledger.id, rows);
                    res.push(LedgerAccount::Intermediate(Ledger::<
                        ledger::intermediate::ActiveModel,
                    >::new(
                        ledger, rows[0]
                    )))
                }
                LedgerType::Leaf => {
                    let rows = <R as RepositoryOperations<
                        ledger::leaf::Model,
                        ledger::leaf::ActiveModel,
                        LedgerId,
                    >>::get(self.store(), Some(&vec![ledger.id]))
                    .await?;
                    res.push(LedgerAccount::Leaf(
                        Ledger::<ledger::leaf::ActiveModel>::new(ledger, rows[0]),
                    ))
                }
            }
        }

        Ok(res)
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
            <R as RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>>::get(
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
        Ok(<R as RepositoryOperations<
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
