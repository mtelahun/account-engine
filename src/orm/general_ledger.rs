use async_trait::async_trait;

use crate::entity::{accounting_period, general_ledger, journal, ledger};

use super::{OrmError, Resource};

#[async_trait]
pub trait GeneralLedgerService {
    async fn init(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError>;

    async fn add_journal(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError>;

    async fn add_ledger(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError>;

    async fn add_period(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError>;
}

impl Resource for general_ledger::ActiveModel {
    const NAME: &'static str = "general_ledger";
}
