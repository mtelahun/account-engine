use async_trait::async_trait;
use journal_entry::LedgerKey;

use crate::{
    domain::{AccountId, XactType},
    resource::{account_engine::AccountEngine, ledger, ledger::journal_entry, PostingRef},
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Store,
};

use super::ServiceError;

#[async_trait]
pub trait LedgerService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
        + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, AccountId>
        + ResourceOperations<
            ledger::intermediate::Model,
            ledger::intermediate::ActiveModel,
            AccountId,
        > + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
        + ResourceOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + Sync
        + Send,
{
    fn store(&self) -> &R;

    async fn journal_entries(
        &self,
        id: AccountId,
    ) -> Result<Vec<ledger::journal_entry::ActiveModel>, ServiceError> {
        let mut res = Vec::<ledger::journal_entry::ActiveModel>::new();
        let entries = self.store().journal_entries_by_ledger(&[id]).await?;
        let xacts = self.store().journal_entry_ledgers_by_ledger(&[id]).await?;
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
            let counterpart = self.journal_entry_by_key(key).await?;
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
        let entry: Vec<ledger::transaction::ActiveModel> =
            self.store().get(Some(&vec![posting_ref.key])).await?;
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
        let xact: Vec<ledger::transaction::ledger::ActiveModel> =
            self.store().get(Some(&vec![posting_ref.key])).await?;
        for t in xact {
            if t.ledger_dr_id == posting_ref.account_id {
                if let Some(counterpart) = self
                    .journal_entry_by_key(LedgerKey {
                        ledger_id: t.ledger_id,
                        timestamp: t.timestamp,
                    })
                    .await?
                {
                    return Ok(Some(ledger::journal_entry::ActiveModel {
                        ledger_id: t.ledger_dr_id,
                        timestamp: t.timestamp,
                        xact_type: XactType::Dr,
                        amount: counterpart.amount,
                        journal_ref: counterpart.journal_ref,
                    }));
                }
            }
        }

        Ok(None)
    }

    async fn journal_entry_by_key(
        &self,
        key: LedgerKey,
    ) -> Result<Option<ledger::transaction::ActiveModel>, ServiceError> {
        let res: Vec<ledger::transaction::ActiveModel> = self.store().get(Some(&vec![key])).await?;

        match res.len() {
            0 => return Ok(None),
            _ => return Ok(Some(res[0])),
        }
    }
}

#[async_trait]
impl LedgerService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl LedgerService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
