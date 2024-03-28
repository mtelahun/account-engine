use async_trait::async_trait;
use journal_entry::LedgerKey;

use crate::{
    domain::{ArrayString128, ArrayString24, ArrayString3, LedgerId, XactType},
    infrastructure::data::db_context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::ResourceOperations,
    },
    resource::{
        account_engine::AccountEngine,
        ledger::{self, journal_entry, LedgerType},
        AccountBalance, LedgerPostingRef,
    },
    Store,
};

use super::ServiceError;

pub trait LedgerOps {}

#[async_trait]
pub trait LedgerService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
        + ResourceOperations<ledger::intermediate::Model, ledger::intermediate::ActiveModel, LedgerId>
        + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
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
        id: LedgerId,
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
        posting_ref: LedgerPostingRef,
    ) -> Result<Option<ledger::journal_entry::ActiveModel>, ServiceError> {
        let entry: Vec<ledger::transaction::ActiveModel> =
            self.store().get(Some(&vec![posting_ref.key])).await?;
        for e in entry.iter() {
            if e.ledger_id == posting_ref.ledger_id {
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
            if t.ledger_dr_id == posting_ref.ledger_id {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ledger<S: LedgerOps> {
    pub id: LedgerId,
    pub number: ArrayString24,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayString128,
    pub currency_code: Option<ArrayString3>,
    extra: S,
}

#[derive(Clone, Copy, Debug)]
pub enum LedgerAccount {
    Derived(Ledger<ledger::derived::ActiveModel>),
    Intermediate(Ledger<ledger::intermediate::ActiveModel>),
    Leaf(Ledger<ledger::leaf::ActiveModel>),
}

impl LedgerOps for ledger::derived::ActiveModel {}

impl LedgerOps for ledger::intermediate::ActiveModel {}

impl LedgerOps for ledger::leaf::ActiveModel {}

impl Ledger<ledger::derived::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}

impl Ledger<ledger::intermediate::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}

impl Ledger<ledger::leaf::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}
impl<S: LedgerOps> Ledger<S> {
    pub fn new(l: ledger::ActiveModel, s: S) -> Ledger<S> {
        Self {
            id: l.id,
            number: l.number,
            ledger_type: l.ledger_type,
            parent_id: l.parent_id,
            name: l.name,
            currency_code: l.currency_code,
            extra: s,
        }
    }
}

impl LedgerAccount {
    pub fn id(&self) -> LedgerId {
        match self {
            LedgerAccount::Derived(l) => l.id,
            LedgerAccount::Intermediate(l) => l.id,
            LedgerAccount::Leaf(l) => l.id,
        }
    }

    pub fn name(&self) -> ArrayString128 {
        match self {
            LedgerAccount::Derived(l) => l.name,
            LedgerAccount::Intermediate(l) => l.name,
            LedgerAccount::Leaf(l) => l.name,
        }
    }

    pub fn number(&self) -> ArrayString24 {
        match self {
            LedgerAccount::Derived(l) => l.number,
            LedgerAccount::Intermediate(l) => l.number,
            LedgerAccount::Leaf(l) => l.number,
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
