use async_trait::async_trait;

use crate::{
    domain::SubJournalTemplateColId,
    resource::journal,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::special::template::column::Model,
        journal::transaction::special::template::column::ActiveModel,
        SubJournalTemplateColId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::special::template::column::Model,
    ) -> Result<journal::transaction::special::template::column::ActiveModel, OrmError> {
        let tpl: journal::transaction::special::template::column::ActiveModel = model.into();
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .journal_xact_sub_template_column
            .iter()
            .any(|(k, _)| *k == tpl.id);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate journal template column: {}",
                tpl.id
            )));
        }
        inner.journal_xact_sub_template_column.insert(tpl.id, tpl);

        Ok(tpl)
    }

    async fn get(
        &self,
        ids: Option<&Vec<SubJournalTemplateColId>>,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::special::template::column::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, v) in inner.journal_xact_sub_template_column.iter() {
                if ids.iter().any(|id| id == key) {
                    res.push(*v)
                }
            }
        } else {
            for v in inner.journal_xact_sub_template_column.values() {
                res.push(*v)
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::template::column::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: SubJournalTemplateColId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: SubJournalTemplateColId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: SubJournalTemplateColId) -> Result<u64, OrmError> {
        todo!()
    }
}
