use async_trait::async_trait;

use crate::{
    domain::SpecialJournalTemplateId,
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::ResourceOperations,
    },
    resource::journal,
};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::template::Model,
        journal::transaction::special::template::ActiveModel,
        SpecialJournalTemplateId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::template::Model,
    ) -> Result<journal::transaction::special::template::ActiveModel, OrmError> {
        let tpl: journal::transaction::special::template::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .journal_xact_sub_template
            .iter()
            .any(|(k, _)| *k == tpl.id);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate journal template: {}",
                tpl.id
            )));
        }
        inner.journal_xact_sub_template.insert(tpl.id, tpl);

        Ok(tpl)
    }

    async fn get(
        &self,
        ids: Option<&Vec<SpecialJournalTemplateId>>,
    ) -> Result<Vec<journal::transaction::special::template::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::template::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, v) in inner.journal_xact_sub_template.iter() {
                if ids.iter().any(|id| id == key) {
                    res.push(*v)
                }
            }
        } else {
            for v in inner.journal_xact_sub_template.values() {
                res.push(*v)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::template::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::template::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: SpecialJournalTemplateId) -> Result<u64, OrmError> {
        todo!()
    }
}
