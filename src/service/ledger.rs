use std::str::FromStr;

use async_trait::async_trait;
use journal_entry::LedgerKey;
use ledger::LedgerType;

use crate::{
    domain::{AccountId, ArrayShortString, XactType},
    entity::{account_engine::AccountEngine, ledger, ledger::journal_entry, PostingRef},
    resource::{postgres::repository::PostgresRepository, ResourceOperations},
    Repository,
};

use super::ServiceError;

#[async_trait]
pub trait LedgerService<R>
where
    R: Repository
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
        + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, AccountId>
        + ResourceOperations<
            ledger::intermediate::Model,
            ledger::intermediate::ActiveModel,
            AccountId,
        > + Sync
        + Send,
{
    fn repository(&self) -> &R;

    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, ServiceError> {
        let parent: Vec<ledger::ActiveModel> = match model.parent_id {
            Some(id) => self.repository().get(Some(&vec![id])).await?,
            None => return Err(ServiceError::Validation("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(ServiceError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        if model.ledger_no != ArrayShortString::from_str("0").unwrap()
            && !self
                .repository()
                .find_ledger_by_model(model)
                .await?
                .is_empty()
        {
            return Err(ServiceError::Validation(format!(
                "duplicate ledger number: {}",
                model.ledger_no
            )));
        }
        let ledger = self.repository().insert(model).await?;
        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger::intermediate::Model { id: ledger.id };
            let _ = self.repository().insert(&intermediate).await?;
        } else {
            let account = ledger::leaf::Model { id: ledger.id };
            let _ = self.repository().insert(&account).await?;
        }

        Ok(ledger)
    }

    async fn journal_entries(
        &self,
        id: AccountId,
    ) -> Result<Vec<ledger::journal_entry::ActiveModel>, ServiceError> {
        let mut res = Vec::<ledger::journal_entry::ActiveModel>::new();
        let entries = self.repository().ledger_transactions_by_ledger_id(id).await;
        let xacts = self.repository().ledger_transaction_by_dr(id).await;
        for e in entries {
            res.push(ledger::journal_entry::ActiveModel {
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
            let counterpart = self.repository().ledger_line_by_key(key).await;
            if let Some(counterpart) = counterpart {
                res.push(ledger::journal_entry::ActiveModel {
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
    ) -> Result<Option<ledger::journal_entry::ActiveModel>, ServiceError> {
        let entry = self
            .repository()
            .find_ledger_line(&Some(vec![posting_ref.key]))
            .await?;
        for e in entry.iter() {
            if e.ledger_id == posting_ref.account_id {
                return Ok(Some(ledger::journal_entry::ActiveModel {
                    ledger_id: e.ledger_id,
                    timestamp: e.timestamp,
                    xact_type: XactType::Cr,
                    amount: e.amount,
                    journal_ref: e.journal_ref,
                }));
            }
        }
        let xact = self
            .repository()
            .find_ledger_transaction(&Some(vec![posting_ref.key]))
            .await?;
        for t in xact {
            if t.ledger_dr_id == posting_ref.account_id {
                let counterpart = self
                    .repository()
                    .ledger_line_by_key(LedgerKey {
                        ledger_id: t.ledger_id,
                        timestamp: t.timestamp,
                    })
                    .await
                    .unwrap();
                return Ok(Some(ledger::journal_entry::ActiveModel {
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

#[async_trait]
impl LedgerService<PostgresRepository> for AccountEngine<PostgresRepository> {
    fn repository(&self) -> &PostgresRepository {
        &self.repository
    }
}
