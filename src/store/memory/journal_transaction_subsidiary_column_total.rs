use async_trait::async_trait;

use crate::{
    domain::ColumnTotalId,
    resource::journal,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::column::total::Model,
        journal::transaction::special::column::total::ActiveModel,
        ColumnTotalId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::column::total::Model,
    ) -> Result<journal::transaction::special::column::total::ActiveModel, OrmError> {
        let active_model =
            journal::transaction::special::column::total::ActiveModel::from_model(model);
        let mut inner = self.inner.write().await;
        if inner
            .journal_xact_special_colum_total
            .contains_key(&active_model.id)
        {
            return Err(OrmError::Internal(format!(
                "duplicate column total id: {}",
                active_model.id
            )));
        }
        inner
            .journal_xact_special_colum_total
            .insert(active_model.id, active_model);

        Ok(active_model)
    }

    async fn get(
        &self,
        ids: Option<&Vec<ColumnTotalId>>,
    ) -> Result<Vec<journal::transaction::special::column::total::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::column::total::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, value) in inner.journal_xact_special_colum_total.iter() {
                if ids.iter().any(|id| id == key) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.journal_xact_special_colum_total.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::column::total::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        model: &journal::transaction::special::column::total::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            inner.journal_xact_special_colum_total.entry(model.id)
        {
            e.insert(*model);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "column total: {}",
            model.id
        )));
    }

    async fn delete(&self, _id: ColumnTotalId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: ColumnTotalId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: ColumnTotalId) -> Result<u64, OrmError> {
        todo!()
    }
}
