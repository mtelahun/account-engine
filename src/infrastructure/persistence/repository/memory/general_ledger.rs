use async_trait::async_trait;

use crate::{
    domain::general_ledger::general_ledger_id::GeneralLedgerId,
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::general_ledger,
};

#[async_trait]
impl RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
    for MemoryStore
{
    async fn insert(
        &self,
        _model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        return Err(OrmError::NotImplemented(
            "insert() for General Ledger resource".into(),
        ));
    }

    async fn get(
        &self,
        ids: Option<&Vec<GeneralLedgerId>>,
    ) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        let mut res = Vec::<general_ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for gl in inner.general_ledger.values() {
                if ids.iter().any(|i| *i == gl.id) {
                    res.push(*gl);
                }
            }
        } else {
            for gl in inner.general_ledger.values() {
                res.push(*gl);
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &general_ledger::ActiveModel) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        let exists = inner
            .general_ledger
            .iter()
            .any(|(k, v)| *k == model.id || (v.id == model.id));
        if exists {
            inner.general_ledger.insert(model.id, *model);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "general ledger: ({}): {}",
            model.name, model.id
        )));
    }

    async fn delete(&self, _id: GeneralLedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: GeneralLedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: GeneralLedgerId) -> Result<u64, OrmError> {
        todo!()
    }
}
