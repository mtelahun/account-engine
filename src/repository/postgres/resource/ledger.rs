use std::str::FromStr;

use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    domain::{AccountId, ArrayLongString, ArrayShortString, XactType},
    entity::{
        journal_entry, ledger, ledger_intermediate, ledger_leaf, LedgerKey, LedgerType, PostingRef,
    },
    orm::{LedgerService, OrmError, Resource, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId> for PostgresRepository {
    async fn get(
        &self,
        ids: Option<&Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1::AccountId[])",
            ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::ActiveModel>::new();
        for row in rows {
            let am = ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn search(&self, _domain: &str) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        todo!()
    }

    async fn insert(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, ledger_no, name, ledger_type, parent_id, currency_code) VALUES($1, $2, $3, $4, $5, $6) RETURNING *;",
            ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                query.as_str(),
                &[
                    &AccountId::new(),
                    &model.ledger_no.as_str(),
                    &model.name.as_str(),
                    &model.ledger_type,
                    &model.parent_id,
                    &model.currency_code,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::ActiveModel::from(res))
    }

    async fn save(&self, model: &ledger::ActiveModel) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET name = $1, currency_code = $2, parent_id = $3, ledger_type = $4, ledger_no = $5 WHERE id = $6::LedgerId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(
            query.as_str(),
            &[
                &model.name,
                &model.currency_code,
                &model.parent_id,
                &model.ledger_type,
                &model.ledger_no,
                &model.id,
            ],
        )
        .await
        .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn delete(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "DELETE FROM {} WHERE id = $1::LedgerId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn archive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = true WHERE id = $1::AccountId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    async fn unarchive(&self, id: AccountId) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET archived = false WHERE id = $1::AccountId;",
            ledger::ActiveModel::NAME
        );

        conn.execute(query.as_str(), &[&id])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

#[async_trait]
impl LedgerService for PostgresRepository {
    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let parent: Vec<ledger::ActiveModel> = match model.parent_id {
            Some(id) => self.get(Some(&vec![id])).await?,
            None => return Err(OrmError::Constraint("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(OrmError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        if model.ledger_no != ArrayShortString::from_str("0").unwrap()
            && !self.find_ledger_by_model(model).await?.is_empty()
        {
            return Err(OrmError::DuplicateRecord(format!(
                "duplicate ledger number: {}",
                model.ledger_no
            )));
        }
        let ledger = self.insert(model).await?;
        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger_intermediate::Model { id: ledger.id };
            let _ = self.insert(&intermediate).await?;
        } else {
            let account = ledger_leaf::Model { id: ledger.id };
            let _ = self.insert(&account).await?;
        }

        Ok(ledger)
    }

    async fn journal_entries(
        &self,
        id: AccountId,
    ) -> Result<Vec<journal_entry::ActiveModel>, OrmError> {
        let mut res = Vec::<journal_entry::ActiveModel>::new();
        let entries = self.ledger_line_by_id(id).await;
        let xacts = self.ledger_transaction_by_dr(id).await;
        for e in entries {
            res.push(journal_entry::ActiveModel {
                ledger_id: e.ledger_id,
                timestamp: e.timestamp,
                xact_type: XactType::Cr,
                amount: e.amount,
                journal_ref: e.journal_ref,
            })
        }
        for t in xacts {
            let key = LedgerKey {
                ledger_id: t.ledger_id,
                timestamp: t.timestamp,
            };
            let counterpart = self.ledger_line_by_key(key).await;
            if let Some(counterpart) = counterpart {
                res.push(journal_entry::ActiveModel {
                    ledger_id: t.ledger_dr_id,
                    timestamp: t.timestamp,
                    xact_type: XactType::Dr,
                    amount: counterpart.amount,
                    journal_ref: counterpart.journal_ref,
                });
            } else {
                // TODO: Log error
                eprintln!("failed to find counterpart journal entry: {key}");
            }
        }

        Ok(res)
    }

    async fn journal_entry_by_posting_ref(
        &self,
        posting_ref: PostingRef,
    ) -> Result<Option<journal_entry::ActiveModel>, OrmError> {
        let entry = self.find_ledger_line(&Some(vec![posting_ref.key])).await?;
        for e in entry.iter() {
            if e.ledger_id == posting_ref.account_id {
                return Ok(Some(journal_entry::ActiveModel {
                    ledger_id: e.ledger_id,
                    timestamp: e.timestamp,
                    xact_type: XactType::Cr,
                    amount: e.amount,
                    journal_ref: e.journal_ref,
                }));
            }
        }
        let xact = self
            .find_ledger_transaction(&Some(vec![posting_ref.key]))
            .await?;
        for t in xact {
            if t.ledger_dr_id == posting_ref.account_id {
                let counterpart = self
                    .ledger_line_by_key(LedgerKey {
                        ledger_id: t.ledger_id,
                        timestamp: t.timestamp,
                    })
                    .await
                    .unwrap();
                return Ok(Some(journal_entry::ActiveModel {
                    ledger_id: t.ledger_dr_id,
                    timestamp: t.timestamp,
                    xact_type: XactType::Dr,
                    amount: counterpart.amount,
                    journal_ref: counterpart.journal_ref,
                }));
            }
        }

        Ok(None)
    }
}

impl From<Row> for ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayLongString::from_str(value.get("name")).unwrap_or_default(),
            ledger_no: ArrayShortString::from_str(value.get("ledger_no")).unwrap_or_default(),
            ledger_type: value.get("ledger_type"),
            parent_id: value.get("parent_id"),
            currency_code: value.get("currency_code"),
        }
    }
}
