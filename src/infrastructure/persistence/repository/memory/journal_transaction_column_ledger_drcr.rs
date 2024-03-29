use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    domain::entity::{
        general_journal::journal_id::JournalId,
        journal_transaction_column::journal_transaction_column_id::JournalTransactionColumnId,
        special_journal_template_column::template_column_id::TemplateColumnId,
    },
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, repository_operations::RepositoryOperations,
    },
    resource::journal,
};

#[async_trait]
impl
    RepositoryOperations<
        journal::transaction::column::ledger_drcr::Model,
        journal::transaction::column::ledger_drcr::ActiveModel,
        JournalTransactionColumnId,
    > for MemoryStore
{
    async fn insert(
        &self,
        model: &journal::transaction::column::ledger_drcr::Model,
    ) -> Result<journal::transaction::column::ledger_drcr::ActiveModel, OrmError> {
        let jtx_col: journal::transaction::column::ledger_drcr::ActiveModel = (*model).into();
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_ledger_drcr.get_mut(&jtx_col.id()) {
            Some(val) => val.push(jtx_col),
            None => {
                inner
                    .journal_xact_column_ledger_drcr
                    .insert(jtx_col.id(), vec![jtx_col]);
            }
        };

        Ok(jtx_col)
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionColumnId>>,
    ) -> Result<Vec<journal::transaction::column::ledger_drcr::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::column::ledger_drcr::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for (key, lst) in inner.journal_xact_column_ledger_drcr.iter() {
                if ids.iter().any(|id| id == key) {
                    for v in lst.iter() {
                        res.push(*v)
                    }
                }
            }
        } else {
            for lst in inner.journal_xact_column_ledger_drcr.values() {
                for v in lst.iter() {
                    res.push(*v)
                }
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        domain: &str,
    ) -> Result<Vec<journal::transaction::column::ledger_drcr::ActiveModel>, OrmError> {
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
        for (_, value) in inner.journal_xact_column_ledger_drcr.iter() {
            for v in value.iter() {
                if v.journal_id == journal_id
                    && v.timestamp == timestamp
                    && v.template_column_id == tpl_col_id
                {
                    result.push(*v)
                }
            }
        }

        Ok(result)
    }

    async fn save(
        &self,
        model: &journal::transaction::column::ledger_drcr::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.journal_xact_column_ledger_drcr.get_mut(&model.id()) {
            Some(val) => {
                for v in val.iter_mut() {
                    if v.id() == model.id() {
                        let _ = std::mem::replace(v, *model);
                    }
                }

                return Ok(1);
            }
            None => {
                return Err(OrmError::RecordNotFound(format!(
                    "journal transaction column ledger_drcr: {}",
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
