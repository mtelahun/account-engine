use async_trait::async_trait;

use crate::entity::accounting_period;

use super::{OrmError, Resource};

#[async_trait]
pub trait AccountingPeriodService {
    async fn create(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError>;
}

impl Resource for accounting_period::ActiveModel {
    const NAME: &'static str = "accounting_period";
}
