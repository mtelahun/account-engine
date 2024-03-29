use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    domain::special_journal::template_column_id::TemplateColumnId,
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
    shared_kernel::{journal_transaction_column_id::JournalTransactionColumnId, JournalId},
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::column::account_cr::Model,
        journal::transaction::column::account_cr::ActiveModel,
        JournalTransactionColumnId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::column::account_cr::Model,
    ) -> Result<journal::transaction::column::account_cr::ActiveModel, OrmError> {
        let jtx_col: journal::transaction::column::account_cr::ActiveModel = (*model).into();
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_account_cr.get_mut(&jtx_col.id()) {
            Some(_) => return Err(OrmError::DuplicateRecord(format!("id {}", jtx_col.id()))),
            None => {
                inner
                    .journal_xact_column_account_cr
                    .insert(jtx_col.id(), jtx_col);
            }
        };

        Ok(jtx_col)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionColumnId>>,
    ) -> Result<Vec<journal::transaction::column::account_cr::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::column::account_cr::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, value) in inner.journal_xact_column_account_cr.iter() {
                if ids.iter().any(|id| id == key) {
                    res.push(*value);
                }
            }
        } else {
            for value in inner.journal_xact_column_account_cr.values() {
                res.push(*value);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        domain: &str,
    ) -> Result<Vec<journal::transaction::column::account_cr::ActiveModel>, OrmError> {
        let lines: Vec<&str> = domain.split(',').collect();
        if lines.len() != 3 {
            return Err(OrmError::Validation("invalid domain".into()));
        }
        let mut journal_id = JournalId::new();
        let mut timestamp = NaiveDateTime::new(NaiveDate::MIN, NaiveTime::MIN);
        let mut tpl_col_id = TemplateColumnId::new();
        for line in lines {
            let terms: Vec<&str> = line.split('=').map(|t| t.trim()).collect();
            if terms.len() != 2 {
                return Err(OrmError::Validation("invalid domain line".into()));
            } else if terms[0].eq("journal_id") {
                journal_id = JournalId::parse_str(terms[1]).map_err(OrmError::Validation)?;
            } else if terms[0].eq("timestamp") {
                timestamp = NaiveDateTime::parse_from_str(terms[1], "%Y-%m-%d %H:%M:%S%.6f")
                    .map_err(|e| OrmError::Validation(e.to_string()))?;
            } else if terms[0].eq("template_column_id") {
                tpl_col_id = TemplateColumnId::parse_str(terms[1])
                    .map_err(|e| OrmError::Validation(e.to_string()))?;
            } else {
                return Err(OrmError::Validation(format!(
                    "unknown term '{}' in domain",
                    terms[0]
                )));
            }
        }
        let mut result = Vec::new();
        let inner = self.inner.read().await;
        for (_, value) in inner.journal_xact_column_account_cr.iter() {
            if value.journal_id == journal_id
                && value.timestamp == timestamp
                && value.template_column_id == tpl_col_id
            {
                result.push(*value)
            }
        }

        Ok(result)
    }

    async fn save(
        &self,
        model: &journal::transaction::column::account_cr::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_account_cr.get_mut(&model.id()) {
            Some(val) => {
                if val.id() == model.id() {
                    let _ = std::mem::replace(val, *model);
                }

                return Ok(1);
            }
            None => {
                return Err(OrmError::RecordNotFound(format!(
                    "journal transaction column account_cr: {}",
                    model.id()
                )))
            }
        };
    }

    async fn delete(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionColumnId) -> Result<u64, OrmError> {
        todo!()
    }
}
