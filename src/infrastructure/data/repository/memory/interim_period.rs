use async_trait::async_trait;

use crate::{
    domain::period::interim_period::InterimPeriodId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::accounting_period::interim_period,
};

#[async_trait]
impl RepositoryOperations<interim_period::Model, interim_period::ActiveModel, InterimPeriodId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &interim_period::Model,
    ) -> Result<interim_period::ActiveModel, OrmError> {
        let id = InterimPeriodId::new();
        let interim = interim_period::ActiveModel {
            id,
            parent_id: model.parent_id,
            start: model.start,
            end: model.end,
        };
        let mut inner = self.inner.write().await;
        inner.interim_period.insert(id, interim);

        Ok(interim)
    }

    async fn get(
        &self,
        ids: Option<&Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_period::ActiveModel>, OrmError> {
        let mut res = Vec::<interim_period::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.interim_period.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.interim_period.values() {
                res.push(*value)
            }
        }

        res.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<interim_period::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &interim_period::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: InterimPeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: InterimPeriodId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: InterimPeriodId) -> Result<u64, OrmError> {
        todo!()
    }
}
