use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
    shared_kernel::JournalId,
};

#[async_trait]
impl RepositoryOperations<journal::Model, journal::ActiveModel, JournalId> for MemoryStore {
    async fn insert(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        let id = JournalId::new();
        let journal = model.into();
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .journal
            .iter()
            .any(|(k, v)| *k == id || (*v.code == model.code));
        if is_duplicate {
            return Err(OrmError::Internal("duplicate Journal Id or Code".into()));
        }
        inner.journal.insert(id, journal);

        Ok(journal)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.journal.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.journal.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<journal::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &journal::ActiveModel) -> Result<u64, OrmError> {
        let journal = journal::ActiveModel { ..*model };
        let mut inner = self.inner.write().await;
        let exists = inner
            .journal
            .iter()
            .any(|(k, v)| *k == model.id || (v.code == model.code));
        if exists {
            inner.journal.insert(model.id, journal);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "journal: ({}): {}",
            model.code, model.id
        )));
    }

    async fn delete(&self, id: JournalId) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.journal.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("journal id: {id}"))),
        }
    }

    async fn archive(&self, _id: JournalId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalId) -> Result<u64, OrmError> {
        todo!()
    }
}
