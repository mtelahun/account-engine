use async_trait::async_trait;
use chronoutil::RelativeDuration;

use crate::{
    domain::ids::PeriodId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::accounting_period,
};

#[async_trait]
impl ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let id = PeriodId::new();
        let period_end =
            model.period_start + RelativeDuration::years(1) + RelativeDuration::days(-1);
        let period = accounting_period::ActiveModel {
            fiscal_year: model.fiscal_year,
            period_start: model.period_start,
            period_type: model.period_type,
            id,
            period_end,
        };

        let mut inner = self.inner.write().await;
        let search_ids = inner.period.get(&period.id);
        let mut search_year = false;
        for value in inner.period.values() {
            if value.fiscal_year == period.fiscal_year {
                search_year = true;
            }
        }
        if search_ids.is_none() && !search_year {
            inner.period.insert(period.id, period);

            return Ok(period);
        }

        Err(OrmError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }

    async fn get(
        &self,
        ids: Option<&Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        let mut res = Vec::<accounting_period::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.period.values() {
                if ids.iter().any(|id| value.id == *id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.period.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _active_model: &accounting_period::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: PeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: PeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: PeriodId) -> Result<u64, OrmError> {
        todo!()
    }
}
