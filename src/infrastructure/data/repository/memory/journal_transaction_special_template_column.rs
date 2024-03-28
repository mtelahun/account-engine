use async_trait::async_trait;

use crate::{
    infrastructure::data::db_context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
    shared_kernel::{SpecialJournalTemplateId, TemplateColumnId},
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::special::template::column::Model,
        journal::transaction::special::template::column::ActiveModel,
        TemplateColumnId,
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
        ids: Option<&Vec<TemplateColumnId>>,
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
        domain: &str,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        let terms: Vec<&str> = domain.split('=').map(|t| t.trim()).collect();
        if terms.len() != 2 {
            return Err(OrmError::Validation("invalid domain".into()));
        } else if terms[0] != "template_id" {
            return Err(OrmError::Validation(format!(
                "unknown term '{}' in domain",
                terms[0]
            )));
        }
        let t_id = SpecialJournalTemplateId::parse_str(terms[1]).map_err(OrmError::Validation)?;
        let mut result = Vec::new();
        let inner = self.inner.read().await;
        for v in inner.journal_xact_sub_template_column.values() {
            if v.template_id == t_id {
                result.push(*v)
            }
        }
        result.sort_by(|a, b| a.sequence.cmp(&b.sequence));

        Ok(result)
    }

    async fn save(
        &self,
        _model: &journal::transaction::special::template::column::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: TemplateColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: TemplateColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: TemplateColumnId) -> Result<u64, OrmError> {
        todo!()
    }
}
