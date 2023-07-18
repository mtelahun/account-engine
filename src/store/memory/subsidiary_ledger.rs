use async_trait::async_trait;

use crate::{
    domain::SubLedgerId,
    resource::subsidiary_ledger,
    store::{OrmError, ResourceOperations},
};

use super::store::MemoryStore;

#[async_trait]
impl ResourceOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
    for MemoryStore
{
    async fn insert(
        &self,
        model: &subsidiary_ledger::Model,
    ) -> Result<subsidiary_ledger::ActiveModel, OrmError> {
        let id = SubLedgerId::new();
        let subledger = subsidiary_ledger::ActiveModel {
            id,
            name: model.name,
            ledger_account_id: model.ledger_account_id,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .subsidary_ledger
            .iter()
            .any(|(k, _)| *k == subledger.id);
        if is_duplicate {
            return Err(OrmError::Internal(format!(
                "duplicate subledger: {}",
                subledger.id
            )));
        }
        inner.subsidary_ledger.insert(subledger.id, subledger);

        Ok(subledger)
    }

    async fn get(
        &self,
        ids: Option<&Vec<SubLedgerId>>,
    ) -> Result<Vec<subsidiary_ledger::ActiveModel>, OrmError> {
        let mut res = Vec::<subsidiary_ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.subsidary_ledger.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.subsidary_ledger.values() {
                res.push(*value)
            }
        }

        Ok(res)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<subsidiary_ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, model: &subsidiary_ledger::ActiveModel) -> Result<u64, OrmError> {
        let subledger = subsidiary_ledger::ActiveModel {
            id: model.id,
            name: model.name,
            ledger_account_id: model.ledger_account_id,
        };
        let mut inner = self.inner.write().await;
        let exists = inner.subsidary_ledger.iter().any(|(k, _)| *k == model.id);
        if exists {
            inner.subsidary_ledger.insert(model.id, subledger);

            return Ok(1);
        }

        return Err(OrmError::RecordNotFound(format!(
            "subledger: ({}): {}",
            model.id, model.name
        )));
    }

    async fn delete(&self, id: SubLedgerId) -> Result<u64, OrmError> {
        let mut inner = self.inner.write().await;
        match inner.subsidary_ledger.remove(&id) {
            Some(_) => return Ok(1),
            None => return Err(OrmError::RecordNotFound(format!("subledger: {id}"))),
        }
    }

    async fn archive(&self, _id: SubLedgerId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: SubLedgerId) -> Result<u64, OrmError> {
        todo!()
    }
}
