use async_trait::async_trait;

use crate::{
    domain::EntityCode,
    resource::external,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<external::entity_type::Model, external::entity_type::ActiveModel, EntityCode>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &external::entity_type::Model,
    ) -> Result<external::entity_type::ActiveModel, OrmError> {
        let entity_type = external::entity_type::ActiveModel {
            code: model.code,
            description: model.description,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner.entity_type.iter().any(|(k, _)| *k == model.code);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate external entity type: {}",
                model.code
            )));
        }
        inner.entity_type.insert(model.code, entity_type);

        Ok(entity_type)
    }

    async fn get(
        &self,
        ids: Option<&Vec<EntityCode>>,
    ) -> Result<Vec<external::entity_type::ActiveModel>, OrmError> {
        let mut res = Vec::<external::entity_type::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.entity_type.values() {
                if ids.iter().any(|id| *id == value.code) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.entity_type.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::entity_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::entity_type::ActiveModel) -> Result<u64, OrmError> {
        let entity = external::entity_type::ActiveModel {
            code: model.code,
            description: model.description,
        };
        let mut inner = self.inner.write().await;
        let exists = inner.entity_type.iter().any(|(k, _)| *k == model.code);
        if exists {
            inner.entity_type.insert(model.code, entity);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "external entity type: ({}): {}",
            model.code, model.description
        )));
    }

    async fn delete(&self, id: EntityCode) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.entity_type.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("account id: {id}"))),
        }
    }

    async fn archive(&self, _id: EntityCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: EntityCode) -> Result<u64, OrmError> {
        todo!()
    }
}
