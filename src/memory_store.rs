use async_trait::async_trait;
use chronoutil::RelativeDuration;
use std::{collections::HashMap, iter::zip, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        ids::{InterimPeriodId, JournalId, PeriodId},
        xact_type::XactType,
        AccountId, ArrayLongString, ArrayShortString, GeneralLedgerId, JournalTransactionId,
        LedgerXactTypeCode,
    },
    entity::{
        accounting_period, general_ledger, interim_accounting_period, journal_entry,
        journal_transaction, journal_transaction_line, journal_transaction_line_account,
        journal_transaction_line_ledger, journal_transaction_record, jrnl::journal, ledger,
        ledger_intermediate, ledger_leaf, ledger_line, ledger_xact_type, transaction::ledger,
        InterimType, LedgerKey, LedgerType, PostingRef, TransactionState,
    },
    orm::{error::OrmError, AccountRepository},
};

#[derive(Clone, Debug, Default)]
struct JournalTxLines {
    list: Vec<journal_transaction_line_ledger::ActiveModel>,
}

impl JournalTxLines {
    pub fn new() -> Self {
        Self {
            list: Vec::<journal_transaction_line_ledger::ActiveModel>::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct JournalTxAccountLines {
    pub list: Vec<journal_transaction_line_account::ActiveModel>,
}

impl JournalTxAccountLines {
    pub fn new() -> Self {
        Self {
            list: Vec::<journal_transaction_line_account::ActiveModel>::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Clone, Debug, Default)]
pub struct Inner {
    general_ledger: HashMap<GeneralLedgerId, general_ledger::ActiveModel>,
    ledger: HashMap<AccountId, ledger::ActiveModel>,
    ledger_intermediate: HashMap<AccountId, ledger_intermediate::ActiveModel>,
    ledger_account: HashMap<AccountId, ledger_leaf::ActiveModel>,
    period: HashMap<PeriodId, accounting_period::ActiveModel>,
    interim_period: HashMap<InterimPeriodId, interim_accounting_period::ActiveModel>,
    journal: HashMap<JournalId, journal::ActiveModel>,
    journal_xact: HashMap<JournalTransactionId, journal_transaction_record::ActiveModel>,
    journal_xact_line: HashMap<JournalTransactionId, JournalTxLines>,
    journal_xact_line_account: HashMap<JournalTransactionId, JournalTxAccountLines>,
    journal_entry: HashMap<LedgerKey, ledger_line::ActiveModel>,
    ledger_xact: HashMap<LedgerKey, transaction::ledger::ActiveModel>,
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
                ledger_id: e.ledger_id,
                timestamp: e.timestamp,
                xact_type: XactType::Cr,
                amount: e.amount,
                journal_ref: e.journal_ref,
            })
        }
        for t in xacts {
            let counterpart = self
                .ledger_entry_by_key(LedgerKey {
                    ledger_id: t.ledger_id,
                    timestamp: t.timestamp,
                })
                .await
                .unwrap();
            res.push(journal_entry::ActiveModel {
                ledger_id: t.ledger_dr_id,
                timestamp: t.timestamp,
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
    ) -> Vec<ledger_line::ActiveModel> {
        let mut res = Vec::<ledger_line::ActiveModel>::new();
        let inner = self.inner.read().await;
        for (key, entry) in &inner.journal_entry {
            if key.ledger_id == account_id {
                res.append(&mut vec![*entry]);
            }
        }

        res
    }

    pub async fn ledger_transactions_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Vec<transaction::ledger::ActiveModel> {
        let mut res = Vec::<transaction::ledger::ActiveModel>::new();
        let inner = self.inner.read().await;
        for tx in inner.ledger_xact.values() {
            if tx.ledger_dr_id == account_id {
                res.push(*tx);
            }
        }

        res
    }

    pub async fn ledger_entry_by_key(&self, key: LedgerKey) -> Option<ledger_line::ActiveModel> {
        let inner = self.inner.read().await;
        if let Some(entry) = inner.journal_entry.get(&key) {
            return Some(*entry);
        };

        None
    }

    pub async fn journal_entry_by_ref(
        &self,
        posting_ref: PostingRef,
    ) -> Option<journal_entry::ActiveModel> {
        let entries = self.journal_entries_by_key(posting_ref.key).await;
        if !entries.is_empty() {
            for entry in entries.iter() {
                if entry.ledger_id == posting_ref.account_id {
                    return Some(*entry);
                }
            }
        }

        None
    }

    async fn journal_entries_by_key(&self, key: LedgerKey) -> Vec<journal_entry::ActiveModel> {
        let mut res = Vec::<journal_entry::ActiveModel>::new();
        let le = self.ledger_entry_by_key(key).await.unwrap();
        let lt = self.ledger_transaction_by_key(key).await.unwrap();
        res.push(journal_entry::ActiveModel {
            ledger_id: le.ledger_id,
            timestamp: le.timestamp,
            xact_type: XactType::Cr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });
        res.push(journal_entry::ActiveModel {
            ledger_id: lt.ledger_dr_id,
            timestamp: lt.timestamp,
            xact_type: XactType::Dr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });

        res
    }

    async fn ledger_transaction_by_key(
        &self,
        key: LedgerKey,
    ) -> Option<transaction::ledger::ActiveModel> {
        let inner = self.inner.read().await;
        for tx in inner.ledger_xact.values() {
            if tx.ledger_id == key.ledger_id && tx.timestamp == key.timestamp {
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
        let xact_lines = match inner.journal_xact_line.get_mut(&jxact_id) {
            None => return false,
            Some(value) => &mut value.list,
        };
        let xact_lines_copy = xact_lines.clone();

        let mut entry_list = HashMap::<LedgerKey, ledger_line::ActiveModel>::new();
        let mut ledger_xact_list = HashMap::<LedgerKey, transaction::ledger::ActiveModel>::new();
        let mut ledger_posted_list =
            Vec::<(LedgerKey, &journal_transaction_line_ledger::ActiveModel)>::new();
        let cr_xact_lines = xact_lines_copy
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines = xact_lines_copy
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
            let key = LedgerKey {
                ledger_id: cr.ledger_id,
                timestamp: cr.timestamp,
            };
            let entry = ledger_line::ActiveModel {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: cr.amount,
                journal_ref: jxact_id,
            };
            let tx_dr = transaction::ledger::ActiveModel {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_dr_id: dr.ledger_id,
            };
            entry_list.insert(key, entry);
            ledger_xact_list.insert(key, tx_dr);
            ledger_posted_list.push((key, dr));
            ledger_posted_list.push((key, cr));
        }

        for line in xact_lines.iter_mut() {
            for (key, post_line) in ledger_posted_list.iter() {
                if *line == **post_line {
                    line.state = TransactionState::Posted;
                    line.posting_ref = Some(PostingRef {
                        key: *key,
                        account_id: line.ledger_id,
                    });
                }
            }
        }
        for (k, e) in entry_list.iter() {
            inner.journal_entry.insert(*k, *e);
        }
        for (k, tx_dr) in ledger_xact_list.iter() {
            inner.ledger_xact.insert(*k, *tx_dr);
        }

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
            general_ledger: HashMap::<GeneralLedgerId, general_ledger::ActiveModel>::new(),
            ledger: HashMap::<AccountId, ledger::ActiveModel>::new(),
            ledger_intermediate: HashMap::<AccountId, ledger_intermediate::ActiveModel>::new(),
            ledger_account: HashMap::<AccountId, ledger_leaf::ActiveModel>::new(),
            period: HashMap::<PeriodId, accounting_period::ActiveModel>::new(),
            interim_period: HashMap::<InterimPeriodId, interim_accounting_period::ActiveModel>::new(
            ),
            journal: HashMap::<JournalId, journal::ActiveModel>::new(),
            journal_xact:
                HashMap::<JournalTransactionId, journal_transaction_record::ActiveModel>::new(),
            journal_xact_line: HashMap::<JournalTransactionId, JournalTxLines>::new(),
            journal_xact_line_account: HashMap::<JournalTransactionId, JournalTxAccountLines>::new(
            ),
            journal_entry: HashMap::<LedgerKey, ledger_line::ActiveModel>::new(),
            ledger_xact: HashMap::<LedgerKey, transaction::ledger::ActiveModel>::new(),
            // _ext_account_txs: HashMap::<LedgerKey, ExternalTransaction>::new(),
            ledger_xact_type: HashMap::<LedgerXactTypeCode, ledger_xact_type::ActiveModel>::new(),
        };
        res.ledger_xact_type
            .insert(code, ledger_xact_type::ActiveModel { code });
        let root_id = AccountId::new();
        let root = ledger::ActiveModel {
            id: root_id,
            ledger_no: ArrayShortString::from_str("0").unwrap(),
            ledger_type: LedgerType::Intermediate,
            parent_id: None,
            name: ArrayLongString::from_str("Root").unwrap(),
            currency_code: None,
        };
        res.ledger.insert(root_id, root);

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
        let mut search_year = false;
        for value in inner.period.values() {
            if value.fiscal_year == period.fiscal_year {
                search_year = true;
            }
        }
        if search_ids.is_none() && !search_year {
            inner.period.insert(period.id, period);

            return Ok(period);
        }

        Err(OrmError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }

    async fn search(
        &self,
        ids: Option<Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
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

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[PeriodId],
        _model: &accounting_period::ActiveModel,
    ) -> Result<u64, OrmError> {
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
        ids: Option<Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_accounting_period::ActiveModel>, OrmError> {
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
        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[InterimPeriodId],
        _model: &interim_accounting_period::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
    for MemoryStore
{
    async fn create(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let gl_id = GeneralLedgerId::new();
        let mut inner = self.inner.write().await;
        let mut root_id: Option<AccountId> = None;
        for ledger in inner.ledger.values() {
            if ledger.ledger_no == ArrayShortString::from_str("0").unwrap() {
                root_id = Some(ledger.id);
                break;
            }
        }

        if let Some(id) = root_id {
            let gl = general_ledger::ActiveModel {
                id: gl_id,
                name: model.name,
                currency_code: model.currency_code,
                root: id,
            };
            inner.general_ledger.insert(gl_id, gl);

            Ok(gl)
        } else {
            Err(OrmError::RecordNotFound(
                "unable to find root ledger".into(),
            ))
        }
    }

    async fn search(
        &self,
        ids: Option<Vec<GeneralLedgerId>>,
    ) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
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

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[GeneralLedgerId],
        _model: &general_ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<ledger::Model, ledger::ActiveModel, AccountId> for MemoryStore {
    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let parent = match model.parent_id {
            Some(id) => self.search(Some(vec![id])).await?,
            None => return Err(OrmError::Constraint("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(OrmError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        let ledger = ledger::ActiveModel {
            id: AccountId::new(),
            ledger_no: model.ledger_no,
            ledger_type: model.ledger_type,
            parent_id: model.parent_id,
            name: model.name,
            currency_code: model.currency_code,
        };
        let mut inner = self.inner.write().await;
        if inner
            .ledger
            .iter()
            .any(|(_k, v)| v.ledger_no == ledger.ledger_no)
        {
            return Err(OrmError::DuplicateRecord(format!(
                "account {}",
                ledger.ledger_no
            )));
        }

        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger_intermediate::ActiveModel { id: ledger.id };
            if inner.ledger_intermediate.contains_key(&intermediate.id) {
                return Err(OrmError::DuplicateRecord(ledger.id.to_string()));
            }
            inner
                .ledger_intermediate
                .insert(intermediate.id, intermediate);
        } else {
            let account = ledger_leaf::ActiveModel { id: ledger.id };
            if inner.ledger_account.contains_key(&account.id) {
                return Err(OrmError::DuplicateRecord(ledger.id.to_string()));
            }
            inner.ledger_account.insert(account.id, account);
        }
        inner.ledger.insert(ledger.id, ledger);

        Ok(ledger)
    }

    async fn search(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
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

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[AccountId],
        _model: &ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
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
        };
        let mut inner = self.inner.write().await;
        let is_duplicate = inner
            .journal
            .iter()
            .any(|(k, v)| *k == id || (*v.code == model.code));
        if is_duplicate {
            return Err(OrmError::DuplicateRecord(
                "duplicate Journal Id or Code".into(),
            ));
        }
        inner.journal.insert(id, journal.clone());

        Ok(journal)
    }

    async fn search(
        &self,
        ids: Option<Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
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

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[JournalId],
        _model: &journal::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl
    AccountRepository<
        journal_transaction::Model,
        journal_transaction::ActiveModel,
        JournalTransactionId,
    > for MemoryStore
{
    async fn create(
        &self,
        model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        for line in model.lines.iter() {
            if line.ledger_id.is_none() && line.account_id.is_none() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() && line.account_id.is_some() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields NOT empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() {
                if self
                    .search(Some(vec![line.ledger_id.unwrap()]))
                    .await?
                    .is_empty()
                {
                    return Err(OrmError::RecordNotFound(format!(
                        "account id: {}",
                        line.ledger_id.unwrap()
                    )));
                }
            } else if line.account_id.is_some() {
                //     if self
                //         .search_account(Some(vec![line.account_id.unwrap()]))
                //         .await?
                //         .is_empty()
                //     {
                //         return Err(OrmError::RecordNotFound(format!(
                //             "account id: {}",
                //             line.ledger_id.unwrap()
                //         )));
                //     }
            }
        }
        let mut inner = self.inner.write().await;
        let search = inner.journal_xact.get(&jtx_id);
        if search.is_some() {
            return Err(OrmError::DuplicateRecord(format!(
                "journal transaction exists: {}",
                jtx_id
            )));
        }

        let mut res_tx_ledger_lines = Vec::<journal_transaction_line_ledger::ActiveModel>::new();
        let mut res_tx_account_lines = Vec::<journal_transaction_line_account::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal_transaction_line_ledger::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jxl = inner.journal_xact_line.get_mut(&jtx_id);
                if let Some(jxl) = jxl {
                    jxl.list.push(jtx_line)
                } else {
                    let mut new_value = JournalTxLines::new();
                    new_value.list.push(jtx_line);
                    inner.journal_xact_line.insert(jtx_id, new_value);
                }
                res_tx_ledger_lines.push(journal_transaction_line_ledger::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    ledger_id: jtx_line.ledger_id,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            } else {
                let jtx_line = journal_transaction_line_account::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jxl = inner.journal_xact_line_account.get_mut(&jtx_id);
                if let Some(jxl) = jxl {
                    jxl.list.push(jtx_line)
                } else {
                    let mut new_value = JournalTxAccountLines::new();
                    new_value.list.push(jtx_line);
                    inner.journal_xact_line_account.insert(jtx_id, new_value);
                }
                res_tx_account_lines.push(journal_transaction_line_account::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: jtx_line.timestamp,
                    account_id: jtx_line.account_id,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            }
        }
        let jtx = journal_transaction_record::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let _ = inner.journal_xact.insert(jtx_id, jtx);

        let mut ledger_lines = Vec::<journal_transaction_line::ActiveModel>::new();
        if !res_tx_ledger_lines.is_empty() {
            for line in res_tx_ledger_lines.iter() {
                ledger_lines.push(journal_transaction_line::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: line.timestamp,
                    ledger_id: Some(line.ledger_id),
                    account_id: None,
                    xact_type: line.xact_type,
                    amount: line.amount,
                    state: line.state,
                    posting_ref: line.posting_ref,
                });
            }
            let _ = inner.journal_xact_line.insert(
                jtx_id,
                JournalTxLines {
                    list: res_tx_ledger_lines,
                },
            );
        }
        if !res_tx_account_lines.is_empty() {
            let _ = inner.journal_xact_line_account.insert(
                jtx_id,
                JournalTxAccountLines {
                    list: res_tx_account_lines,
                },
            );
        }

        Ok(journal_transaction::ActiveModel {
            journal_id: jtx.journal_id,
            timestamp: jtx.timestamp,
            explanation: jtx.explanation,
            lines: ledger_lines,
        })
    }

    async fn search(
        &self,
        ids: Option<Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction::ActiveModel>, OrmError> {
        let mut res = Vec::<journal_transaction::ActiveModel>::new();
        let inner = self.inner.read().await;

        if let Some(ids) = ids {
            for value in inner.journal_xact.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    let mut xact_lines = Vec::<journal_transaction_line::ActiveModel>::new();
                    for txl in inner.journal_xact_line.values() {
                        if ids.iter().any(|id| *id == value.id()) {
                            for line in txl.list.iter() {
                                xact_lines.push(journal_transaction_line::ActiveModel {
                                    journal_id: line.journal_id,
                                    timestamp: line.timestamp,
                                    ledger_id: Some(line.ledger_id),
                                    account_id: None,
                                    xact_type: line.xact_type,
                                    amount: line.amount,
                                    state: line.state,
                                    posting_ref: line.posting_ref,
                                })
                            }
                        }
                    }
                    let tx = journal_transaction::ActiveModel {
                        journal_id: value.journal_id,
                        timestamp: value.timestamp,
                        explanation: value.explanation,
                        lines: xact_lines,
                    };
                    res.push(tx);
                }
            }
        } else {
            for value in inner.journal_xact.values() {
                let mut xact_lines = Vec::<journal_transaction_line::ActiveModel>::new();
                for txl in inner.journal_xact_line.values() {
                    for line in txl.list.iter() {
                        xact_lines.push(journal_transaction_line::ActiveModel {
                            journal_id: line.journal_id,
                            timestamp: line.timestamp,
                            ledger_id: Some(line.ledger_id),
                            account_id: None,
                            xact_type: line.xact_type,
                            amount: line.amount,
                            state: line.state,
                            posting_ref: line.posting_ref,
                        })
                    }
                }
                let tx = journal_transaction::ActiveModel {
                    journal_id: value.journal_id,
                    timestamp: value.timestamp,
                    explanation: value.explanation,
                    lines: xact_lines,
                };
                res.push(tx);
            }
        }

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[JournalTransactionId],
        _model: &journal_transaction::ActiveModel,
    ) -> Result<u64, OrmError> {
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
