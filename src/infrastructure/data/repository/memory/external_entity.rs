use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::external,
    shared_kernel::ids::ExternalEntityId,
};

#[async_trait]
impl RepositoryOperations<external::entity::Model, external::entity::ActiveModel, ExternalEntityId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &external::entity::Model,
    ) -> Result<external::entity::ActiveModel, OrmError> {
        let id = ExternalEntityId::new();
        let entity = external::entity::ActiveModel {
            id,
            entity_type_code: model.entity_type_code,
            name: model.name,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner.external_entity.iter().any(|(k, _)| *k == id);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate external entity: {}",
                id
            )));
        }
        inner.external_entity.insert(id, entity);

        Ok(entity)
    }

    async fn get(
        &self,
        ids: Option<&Vec<ExternalEntityId>>,
    ) -> Result<Vec<external::entity::ActiveModel>, OrmError> {
        let mut res = Vec::<external::entity::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.external_entity.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.external_entity.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<external::entity::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::entity::ActiveModel) -> Result<u64, OrmError> {
        let entity = external::entity::ActiveModel {
            id: model.id,
            entity_type_code: model.entity_type_code,
            name: model.name,
        };
        let mut inner = self.inner.write().await;
        let exists = inner.external_entity.iter().any(|(k, _)| *k == model.id);
        if exists {
            inner.external_entity.insert(model.id, entity);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "external entity: ({}): {}",
            model.name, model.id
        )));
    }

    async fn delete(&self, id: ExternalEntityId) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.external_entity.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("account id: {id}"))),
        }
    }

    async fn archive(&self, _id: ExternalEntityId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: ExternalEntityId) -> Result<u64, OrmError> {
        todo!()
    }
}
