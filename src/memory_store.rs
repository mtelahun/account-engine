use arrayvec::ArrayString;
use async_trait::async_trait;
use chronoutil::RelativeDuration;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        ids::{InterimPeriodId, JournalId, JournalTransactionId, PeriodId},
        xact_type::XactType,
        AccountId, LedgerId, LedgerXactTypeCode,
    },
    entity::{
        accounting_period, general_ledger, interim_accounting_period, journal_entry,
        jrnl::{journal, transaction::journal_line},
        ledger, ledger_entry, ledger_intermediate, ledger_leaf, ledger_transaction,
        ledger_xact_type, InterimType, LedgerKey, LedgerType, PostingRef, TransactionState,
    },
    orm::{error::OrmError, AccountRepository},
};

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Clone, Debug, Default)]
pub struct Inner {
    general_ledger: HashMap<LedgerId, general_ledger::ActiveModel>,
    ledger: HashMap<AccountId, ledger::ActiveModel>,
    ledger_intermediate: HashMap<AccountId, ledger_intermediate::ActiveModel>,
    ledger_account: HashMap<AccountId, ledger_leaf::ActiveModel>,
    period: HashMap<PeriodId, accounting_period::ActiveModel>,
    interim_period: HashMap<InterimPeriodId, interim_accounting_period::ActiveModel>,
    journal: HashMap<JournalId, journal::ActiveModel>,
    journal_xact: HashMap<JournalTransactionId, journal_line::ActiveModel>,
    journal_entry: HashMap<LedgerKey, ledger_entry::ActiveModel>,
    ledger_xact: HashMap<LedgerKey, ledger_transaction::ActiveModel>,
    // _ext_account_txs: HashMap<LedgerKey, ExternalTransaction>,
    ledger_xact_type: HashMap<LedgerXactTypeCode, ledger_xact_type::ActiveModel>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        Self {
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    pub async fn journal_entries_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Vec<journal_entry::ActiveModel> {
        let mut res = Vec::<journal_entry::ActiveModel>::new();
        let entries = self.ledger_entries_by_account_id(account_id).await;
        let xacts = self.ledger_transactions_by_account_id(account_id).await;
        for e in entries {
            res.push(journal_entry::ActiveModel {
                ledger_no: e.ledger_no,
                datetime: e.datetime,
                xact_type: XactType::Cr,
                amount: e.amount,
                journal_ref: e.journal_ref,
            })
        }
        for t in xacts {
            let counterpart = self
                .ledger_entry_by_key(LedgerKey {
                    ledger_no: t.ledger_no,
                    datetime: t.datetime,
                })
                .await
                .unwrap();
            res.push(journal_entry::ActiveModel {
                ledger_no: t.ledger_dr,
                datetime: t.datetime,
                xact_type: XactType::Dr,
                amount: counterpart.amount,
                journal_ref: counterpart.journal_ref,
            })
        }

        res
    }

    pub async fn ledger_entries_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger_entry::ActiveModel> {
        let mut res = Vec::<ledger_entry::ActiveModel>::new();
        let inner = self.inner.read().await;
        for (key, entry) in &inner.journal_entry {
            if key.ledger_no == account_id {
                res.append(&mut vec![*entry]);
            }
        }

        res
    }

    pub async fn ledger_transactions_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger_transaction::ActiveModel> {
        let mut res = Vec::<ledger_transaction::ActiveModel>::new();
        let inner = self.inner.read().await;
        for tx in inner.ledger_xact.values() {
            if tx.ledger_dr == account_id {
                res.push(*tx);
            }
        }

        res
    }

    pub async fn ledger_entry_by_key(&self, key: LedgerKey) -> Option<ledger_entry::ActiveModel> {
        let inner = self.inner.read().await;
        if let Some(entry) = inner.journal_entry.get(&key) {
            return Some(*entry);
        };

        None
    }

    pub async fn journal_entries_by_ref(
        &self,
        posting_ref: PostingRef,
    ) -> Vec<journal_entry::ActiveModel> {
        self.journal_entries_by_key(posting_ref.ledger_key()).await
    }

    async fn journal_entries_by_key(&self, key: LedgerKey) -> Vec<journal_entry::ActiveModel> {
        let mut res = Vec::<journal_entry::ActiveModel>::new();
        let le = self.ledger_entry_by_key(key).await.unwrap();
        let lt = self.ledger_transaction_by_key(key).await.unwrap();
        res.push(journal_entry::ActiveModel {
            ledger_no: le.ledger_no,
            datetime: le.datetime,
            xact_type: XactType::Cr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });
        res.push(journal_entry::ActiveModel {
            ledger_no: lt.ledger_dr,
            datetime: lt.datetime,
            xact_type: XactType::Dr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });

        res
    }

    async fn ledger_transaction_by_key(
        &self,
        key: LedgerKey,
    ) -> Option<ledger_transaction::ActiveModel> {
        let inner = self.inner.read().await;
        for tx in inner.ledger_xact.values() {
            if tx.ledger_no == key.ledger_no && tx.datetime == key.datetime {
                return Some(*tx);
            }
        }

        None
    }

    // fn ledger_entry_by_ref(&self, posting_ref: PostingRef) -> Option<LedgerEntry::ActiveModel> {
    //     if let Some(res) = self.ledger_entry_by_key(posting_ref.ledger_key()) {
    //         return Some(res);
    //     }

    //     None
    // }

    pub async fn post_journal_transaction(&self, jxact_id: JournalTransactionId) -> bool {
        let ledger_xact_type = self.get_journal_entry_type(jxact_id).await;

        let mut inner = self.inner.write().await;
        let mut xact = match inner.journal_xact.get_mut(&jxact_id) {
            None => return false,
            Some(value) => value,
        };

        let key = LedgerKey {
            ledger_no: xact.account_cr_id,
            datetime: xact.timestamp,
        };
        xact.posting_ref = Some(PostingRef::new(key, ledger_xact_type));
        xact.state = TransactionState::Posted;

        let entry = ledger_entry::ActiveModel {
            ledger_no: key.ledger_no,
            datetime: xact.timestamp,
            ledger_xact_type,
            amount: xact.amount,
            journal_ref: xact.id,
        };
        let tx_dr = ledger_transaction::ActiveModel {
            ledger_no: key.ledger_no,
            datetime: xact.timestamp,
            ledger_dr: xact.account_dr_id,
        };

        inner.journal_entry.insert(key, entry);
        inner.ledger_xact.insert(key, tx_dr);

        true
    }

    async fn get_journal_entry_type(
        &self,
        _jxact_id: JournalTransactionId,
    ) -> ledger_xact_type::ActiveModel {
        let inner = self.inner.read().await;

        *inner
            .ledger_xact_type
            .get(&LedgerXactTypeCode::from_str("AL").unwrap())
            .unwrap()
    }
}

impl Inner {
    pub fn new() -> Inner {
        let code = LedgerXactTypeCode::from_str("AL").unwrap();
        let mut res = Self {
            general_ledger: HashMap::<LedgerId, general_ledger::ActiveModel>::new(),
            ledger: HashMap::<AccountId, ledger::ActiveModel>::new(),
            ledger_intermediate: HashMap::<AccountId, ledger_intermediate::ActiveModel>::new(),
            ledger_account: HashMap::<AccountId, ledger_leaf::ActiveModel>::new(),
            period: HashMap::<PeriodId, accounting_period::ActiveModel>::new(),
            interim_period: HashMap::<InterimPeriodId, interim_accounting_period::ActiveModel>::new(
            ),
            journal: HashMap::<JournalId, journal::ActiveModel>::new(),
            journal_xact: HashMap::<JournalTransactionId, journal_line::ActiveModel>::new(),
            journal_entry: HashMap::<LedgerKey, ledger_entry::ActiveModel>::new(),
            ledger_xact: HashMap::<LedgerKey, ledger_transaction::ActiveModel>::new(),
            // _ext_account_txs: HashMap::<LedgerKey, ExternalTransaction>::new(),
            ledger_xact_type: HashMap::<LedgerXactTypeCode, ledger_xact_type::ActiveModel>::new(),
        };
        res.ledger_xact_type
            .insert(code, ledger_xact_type::ActiveModel { code });

        res
    }
}

#[async_trait]
impl AccountRepository<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
    for MemoryStore
{
    async fn create(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let id = PeriodId::new();
        let period_end =
            model.period_start + RelativeDuration::years(1) + RelativeDuration::days(-1);
        let period = accounting_period::ActiveModel {
            ledger_id: model.ledger_id,
            fiscal_year: model.fiscal_year,
            period_start: model.period_start,
            period_type: model.period_type,
            id,
            period_end,
        };
        let _ = match model.period_type {
            InterimType::CalendarMonth => period.create_interim_calendar(self),
            InterimType::FourWeek => todo!(),
            InterimType::FourFourFiveWeek => todo!(),
        }
        .await
        .map_err(OrmError::Internal)?;

        let mut inner = self.inner.write().await;
        let search_ids = inner.period.get(&period.id);
        let mut search_ledger = false;
        for value in inner.period.values() {
            if value.ledger_id == period.ledger_id {
                search_ledger = true;
            }
        }
        if search_ids.is_none() && !search_ledger {
            inner.period.insert(period.id, period);

            return Ok(period);
        }

        Err(OrmError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }

    async fn search(&self, ids: Option<&[PeriodId]>) -> Vec<accounting_period::ActiveModel> {
        let mut res = Vec::<accounting_period::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.period.values() {
                if ids.iter().any(|id| value.id == *id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.period.values() {
                res.push(*value)
            }
        }

        res
    }

    async fn update(
        &self,
        _ids: &[PeriodId],
        _model: &accounting_period::Model,
    ) -> Result<(), OrmError> {
        todo!()
    }
}

#[async_trait]
impl
    AccountRepository<
        interim_accounting_period::Model,
        interim_accounting_period::ActiveModel,
        InterimPeriodId,
    > for MemoryStore
{
    async fn create(
        &self,
        model: &interim_accounting_period::Model,
    ) -> Result<interim_accounting_period::ActiveModel, OrmError> {
        let id = InterimPeriodId::new();
        let interim = interim_accounting_period::ActiveModel {
            id,
            parent_id: model.parent_id,
            start: model.start,
            end: model.end,
        };
        let mut inner = self.inner.write().await;
        inner.interim_period.insert(id, interim);

        Ok(interim)
    }

    async fn search(
        &self,
        ids: Option<&[InterimPeriodId]>,
    ) -> Vec<interim_accounting_period::ActiveModel> {
        let mut res = Vec::<interim_accounting_period::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.interim_period.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(*value)
                }
            }
        } else {
            for value in inner.interim_period.values() {
                res.push(*value)
            }
        }

        res.sort_by(|a, b| a.start.cmp(&b.start));
        res
    }

    async fn update(
        &self,
        _ids: &[InterimPeriodId],
        _model: &interim_accounting_period::Model,
    ) -> Result<(), OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<general_ledger::Model, general_ledger::ActiveModel, LedgerId>
    for MemoryStore
{
    async fn create(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let gl_id = LedgerId::new();
        let root = ledger::Model {
            general_ledger_id: gl_id,
            ledger_no: ArrayString::<64>::from("0").unwrap(),
            ledger_type: LedgerType::Intermediate,
            parent_id: None,
            name: model.name,
            currency: None,
        };
        let root = self.create(&root).await?;
        let gl = general_ledger::ActiveModel {
            id: gl_id,
            name: model.name,
            currency: model.currency,
            root: root.id,
        };
        let mut inner = self.inner.write().await;
        inner.general_ledger.insert(gl_id, gl);

        Ok(gl)
    }

    async fn search(&self, ids: Option<&[LedgerId]>) -> Vec<general_ledger::ActiveModel> {
        let mut res = Vec::<general_ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for gl in inner.general_ledger.values() {
                if ids.iter().any(|i| *i == gl.id) {
                    res.push(*gl);
                }
            }
        } else {
            for gl in inner.general_ledger.values() {
                res.push(*gl);
            }
        }

        res
    }

    async fn update(
        &self,
        _ids: &[LedgerId],
        _model: &general_ledger::Model,
    ) -> Result<(), OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<ledger::Model, ledger::ActiveModel, AccountId> for MemoryStore {
    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        if model.parent_id.is_none() && model.ledger_no != ArrayString::<64>::from("0").unwrap() {
            return Err(OrmError::Constraint("ledger has no parent".into()));
        } else if model.ledger_no != ArrayString::<64>::from("0").unwrap() {
            let parent = self.search(Some(&[model.parent_id.unwrap()])).await;
            if parent[0].ledger_type != LedgerType::Intermediate {
                return Err(OrmError::Validation(
                    "parent ledger is not an Intermediate Ledger".into(),
                ));
            } else if model.general_ledger_id != parent[0].general_ledger_id {
                return Err(OrmError::Validation(
                    "child ledger not in same general ledger as parent".into(),
                ));
            }
        }

        let ledger = ledger::ActiveModel {
            id: AccountId::new(),
            general_ledger_id: model.general_ledger_id,
            ledger_no: model.ledger_no,
            ledger_type: model.ledger_type,
            parent_id: model.parent_id,
            name: model.name,
            currency: model.currency,
        };
        let mut inner = self.inner.write().await;
        if inner.ledger.iter().any(|(k, v)| {
            *k == ledger.id
                || (v.general_ledger_id == ledger.general_ledger_id
                    && ledger.ledger_no != ArrayString::<64>::from("0").unwrap()
                    && v.ledger_no == ledger.ledger_no)
        }) {
            return Err(OrmError::DuplicateRecord(format!(
                "account {}",
                ledger.ledger_no
            )));
        }

        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger_intermediate::ActiveModel {
                id: ledger.id,
                ledger_no: ledger.ledger_no,
            };
            if inner.ledger_intermediate.contains_key(&intermediate.id) {
                return Err(OrmError::DuplicateRecord(ledger.id.to_string()));
            }
            inner
                .ledger_intermediate
                .insert(intermediate.id, intermediate);
        } else {
            let account = ledger_leaf::ActiveModel {
                id: ledger.id,
                ledger_no: ledger.ledger_no,
            };
            if inner.ledger_account.contains_key(&account.id) {
                return Err(OrmError::DuplicateRecord(ledger.id.to_string()));
            }
            inner.ledger_account.insert(account.id, account);
        }
        inner.ledger.insert(ledger.id, ledger);

        Ok(ledger)
    }

    async fn search(&self, ids: Option<&[AccountId]>) -> Vec<ledger::ActiveModel> {
        let mut res = Vec::<ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for account in inner.ledger.values() {
                if ids.iter().any(|i| *i == account.id) {
                    res.push(*account);
                }
            }
        } else {
            for account in inner.ledger.values() {
                res.push(*account);
            }
        }

        res
    }

    async fn update(&self, _ids: &[AccountId], _model: &ledger::Model) -> Result<(), OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<journal::Model, journal::ActiveModel, JournalId> for MemoryStore {
    async fn create(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        let id = JournalId::new();
        let journal = journal::ActiveModel {
            id,
            name: model.name.clone(),
            code: model.code.clone(),
            ledger_id: model.ledger_id,
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .journal
            .iter()
            .any(|(k, v)| *k == id || (v.ledger_id == model.ledger_id && *v.code == model.code));
        if is_duplicate {
            return Err(OrmError::DuplicateRecord(
                "duplicate Journal Id or Code".into(),
            ));
        }
        inner.journal.insert(id, journal.clone());

        Ok(journal)
    }

    async fn search(&self, ids: Option<&[JournalId]>) -> Vec<journal::ActiveModel> {
        let mut res = Vec::<journal::ActiveModel>::new();
        let inner = self.inner.read().await;
        if let Some(ids) = ids {
            for value in inner.journal.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.push(value.clone())
                }
            }
        } else {
            for value in inner.journal.values() {
                res.push(value.clone())
            }
        }

        res
    }

    async fn update(&self, _ids: &[JournalId], _model: &journal::Model) -> Result<(), OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<journal_line::Model, journal_line::ActiveModel, JournalTransactionId>
    for MemoryStore
{
    async fn create(
        &self,
        model: &journal_line::Model,
    ) -> Result<journal_line::ActiveModel, OrmError> {
        let inner = self.inner.read().await;
        let is_dr = inner.ledger.contains_key(&model.account_dr_id);
        let is_cr = inner.ledger.contains_key(&model.account_cr_id);
        drop(inner);
        if !is_dr {
            return Err(OrmError::RecordNotFound(format!(
                "account id: {}",
                model.account_dr_id
            )));
        }
        if !is_cr {
            return Err(OrmError::RecordNotFound(format!(
                "account id: {}",
                model.account_cr_id
            )));
        }

        let id = JournalTransactionId::new();
        let jtx = journal_line::ActiveModel {
            id,
            timestamp: model.timestamp,
            journal_id: model.journal_id,
            account_dr_id: model.account_dr_id,
            account_cr_id: model.account_cr_id,
            amount: model.amount,
            state: model.state,
            description: model.description.clone(),
            posting_ref: model.posting_ref,
        };
        let mut inner = self.inner.write().await;
        let search = inner.journal_xact.get(&id);
        if search.is_none() {
            inner.journal_xact.insert(id, jtx.clone());

            return Ok(jtx);
        }

        Err(OrmError::DuplicateRecord(format!(
            "journal transaction exists: {}",
            id
        )))
    }

    async fn search(&self, ids: Option<&[JournalTransactionId]>) -> Vec<journal_line::ActiveModel> {
        let mut res = Vec::<journal_line::ActiveModel>::new();
        let inner = self.inner.read().await;

        if let Some(ids) = ids {
            for value in inner.journal_xact.values() {
                if ids.iter().any(|id| *id == value.id) {
                    res.insert(0, value.clone());
                }
            }
        } else {
            for value in inner.journal_xact.values() {
                res.insert(0, value.clone());
            }
        }

        res
    }

    async fn update(
        &self,
        _ids: &[JournalTransactionId],
        _model: &journal_line::Model,
    ) -> Result<(), OrmError> {
        todo!()
    }
}

// impl AccountEngineStorage for MemoryStore {
// fn journal_transactions_by_ledger(&self, ledger_name: &LedgerId) -> Vec<JournalTransaction> {
//     let mut res = Vec::<JournalTransaction>::new();
//     let inner = self.inner.read().unwrap();
//     for value in inner.journal_txs.values() {
//         if value.journal.ledger == *ledger_name {
//             res.insert(0, value.clone());
//         }
//     }

//     res
// }

// fn journal_transaction_by_id(&self, id: JournalTransactionId) -> Option<JournalTransaction> {
//     let inner = self.inner.read().unwrap();
//     for value in inner.journal_txs.values() {
//         if value.id == id {
//             return Some(value.clone());
//         }
//     }

//     None
// }
// }
