use async_trait::async_trait;

use crate::{
    domain::ExternalXactTypeCode,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::external,
};

#[async_trait]
impl
    RepositoryOperations<
        external::transaction_type::Model,
        external::transaction_type::ActiveModel,
        ExternalXactTypeCode,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &external::transaction_type::Model,
    ) -> Result<external::transaction_type::ActiveModel, OrmError> {
        let tx_type = external::transaction_type::ActiveModel::from(model);
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .external_xact_type
            .iter()
            .any(|(k, _)| *k == tx_type.code);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "transaction type code: {}",
                tx_type.code
            )));
        }
        inner.external_xact_type.insert(tx_type.code, tx_type);

        Ok(tx_type)
    }

    async fn get(
        &self,
        ids: Option<&Vec<ExternalXactTypeCode>>,
    ) -> Result<Vec<external::transaction_type::ActiveModel>, OrmError> {
        let mut res = Vec::<external::transaction_type::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.external_xact_type.values() {
                if ids.iter().any(|id| *id == value.code) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.external_xact_type.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<external::transaction_type::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &external::transaction_type::ActiveModel) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        let exists = inner
            .external_xact_type
            .iter()
            .any(|(k, _)| *k == model.code);
        if exists {
            inner.external_xact_type.insert(model.code, *model);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "external transaction type: ({}): {}",
            model.code, model.description
        )));
    }

    async fn delete(&self, id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.external_xact_type.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("entity code: {id}"))),
        }
    }

    async fn archive(&self, _id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: ExternalXactTypeCode) -> Result<u64, OrmError> {
        todo!()
    }
}
