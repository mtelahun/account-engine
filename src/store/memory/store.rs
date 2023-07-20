use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    domain::{
        entity_code::EntityCode,
        ids::{InterimPeriodId, JournalId},
        ArrayCodeString, ArrayLongString, ArrayShortString, ExternalXactTypeCode, GeneralLedgerId,
        JournalTransactionId, LedgerId, LedgerXactTypeCode, PeriodId, SubLedgerId,
    },
    resource::{
        accounting_period, external, general_ledger,
        journal::{self, transaction::TransactionState},
        ledger::{self, journal_entry::LedgerKey, LedgerType},
        ledger_xact_type, organization, subsidiary_ledger,
    },
    store::OrmError,
    Store,
};

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    pub(crate) inner: Arc<RwLock<Inner>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Inner {
    _name: String,
    _uri: String,
    pub(crate) general_ledger: HashMap<GeneralLedgerId, general_ledger::ActiveModel>,
    pub(crate) ledger: HashMap<LedgerId, ledger::ActiveModel>,
    pub(crate) ledger_intermediate: HashMap<LedgerId, ledger::intermediate::ActiveModel>,
    pub(crate) ledger_account: HashMap<LedgerId, ledger::leaf::ActiveModel>,
    pub(crate) period: HashMap<PeriodId, accounting_period::ActiveModel>,
    pub(crate) interim_period:
        HashMap<InterimPeriodId, accounting_period::interim_period::ActiveModel>,
    pub(crate) journal: HashMap<JournalId, journal::ActiveModel>,
    pub(crate) journal_xact:
        HashMap<JournalTransactionId, journal::transaction::record::ActiveModel>,
    pub(crate) journal_xact_line:
        HashMap<JournalTransactionId, Vec<journal::transaction::line::ledger::ActiveModel>>,
    pub(crate) journal_xact_line_account:
        HashMap<JournalTransactionId, Vec<journal::transaction::line::account::ActiveModel>>,
    pub(crate) journal_entry: HashMap<LedgerKey, ledger::transaction::ActiveModel>,
    pub(crate) ledger_xact_account: HashMap<LedgerKey, ledger::transaction::account::ActiveModel>,
    pub(crate) ledger_xact_ledger: HashMap<LedgerKey, ledger::transaction::ledger::ActiveModel>,
    pub(crate) ledger_xact_type: HashMap<LedgerXactTypeCode, ledger_xact_type::ActiveModel>,
    pub(crate) external_xact_type:
        HashMap<ExternalXactTypeCode, external::transaction_type::ActiveModel>,
    pub(crate) subsidary_ledger: HashMap<SubLedgerId, subsidiary_ledger::ActiveModel>,
    pub(crate) external_account: HashMap<LedgerId, external::account::ActiveModel>,
    pub(crate) _entity_type: HashMap<EntityCode, external::entity_type::ActiveModel>,
}

impl MemoryStore {
    pub fn new(name: &str, uri: &str) -> MemoryStore {
        Self {
            inner: Arc::new(RwLock::new(Inner::new(name, uri))),
        }
    }

    pub async fn new_schema(name: &str, uri: &str) -> Result<Self, OrmError> {
        Ok(Self {
            inner: Arc::new(RwLock::new(Inner::new_schema(name, uri).await.unwrap())),
        })
    }
}

impl Inner {
    pub fn new(name: &str, uri: &str) -> Self {
        let mut res = Self {
            _name: name.to_string(),
            _uri: uri.to_string(),
            general_ledger: HashMap::<GeneralLedgerId, general_ledger::ActiveModel>::new(),
            ledger: HashMap::<LedgerId, ledger::ActiveModel>::new(),
            ledger_intermediate: HashMap::<LedgerId, ledger::intermediate::ActiveModel>::new(),
            ledger_account: HashMap::<LedgerId, ledger::leaf::ActiveModel>::new(),
            period: HashMap::<PeriodId, accounting_period::ActiveModel>::new(),
            interim_period: HashMap::<
                InterimPeriodId,
                accounting_period::interim_period::ActiveModel,
            >::new(),
            journal: HashMap::<JournalId, journal::ActiveModel>::new(),
            journal_xact:
                HashMap::<JournalTransactionId, journal::transaction::record::ActiveModel>::new(),
            journal_xact_line: HashMap::<
                JournalTransactionId,
                Vec<journal::transaction::line::ledger::ActiveModel>,
            >::new(),
            journal_xact_line_account: HashMap::<
                JournalTransactionId,
                Vec<journal::transaction::line::account::ActiveModel>,
            >::new(),
            journal_entry: HashMap::<LedgerKey, ledger::transaction::ActiveModel>::new(),
            ledger_xact_account:
                HashMap::<LedgerKey, ledger::transaction::account::ActiveModel>::new(),
            ledger_xact_ledger: HashMap::<LedgerKey, ledger::transaction::ledger::ActiveModel>::new(
            ),
            ledger_xact_type: HashMap::<LedgerXactTypeCode, ledger_xact_type::ActiveModel>::new(),
            external_xact_type: HashMap::<
                ExternalXactTypeCode,
                external::transaction_type::ActiveModel,
            >::new(),
            subsidary_ledger: HashMap::<SubLedgerId, subsidiary_ledger::ActiveModel>::new(),
            external_account: HashMap::<LedgerId, external::account::ActiveModel>::new(),
            _entity_type: HashMap::<EntityCode, external::entity_type::ActiveModel>::new(),
        };
        let code = LedgerXactTypeCode::from_str("LL").unwrap();
        res.ledger_xact_type.insert(
            code,
            ledger_xact_type::ActiveModel {
                code,
                description: "Ledger-Ledger".into(),
            },
        );
        let root_id = res.insert_ledger_root();
        res.insert_general_leger(root_id);

        res
    }

    fn insert_ledger_root(&mut self) -> LedgerId {
        let root_id = LedgerId::new();
        let root = ledger::ActiveModel {
            id: root_id,
            ledger_no: ArrayShortString::from_str("0").unwrap(),
            ledger_type: LedgerType::Intermediate,
            parent_id: None,
            name: ArrayLongString::from_str("Root").unwrap(),
            currency_code: None,
        };
        self.ledger.insert(root_id, root);

        root_id
    }

    fn insert_general_leger(&mut self, root_id: LedgerId) {
        let id = GeneralLedgerId::new();
        let v = general_ledger::ActiveModel {
            id,
            name: ArrayLongString::from_str("Root").unwrap(),
            currency_code: ArrayCodeString::from_str("USD").unwrap(),
            root: root_id,
        };
        self.general_ledger.insert(id, v);
    }

    pub async fn new_schema(name: &str, uri: &str) -> Result<Self, OrmError> {
        Ok(Self::new(name, uri))
    }
}

#[async_trait]
impl Store for MemoryStore {
    async fn create_schema(&self) -> Result<(), OrmError> {
        Ok(())
    }

    async fn organization(&self) -> Result<organization::ActiveModel, OrmError> {
        todo!()
    }

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::line::ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut dummy = Vec::<journal::transaction::line::ledger::ActiveModel>::new();
        let mut inner = self.inner.write().await;
        let xact_lines = match inner.journal_xact_line.get_mut(&id) {
            Some(lines) => lines,
            None => &mut dummy,
        };
        let count = xact_lines.len();
        for mut jl in xact_lines.iter_mut() {
            if jl.ledger_id == line.ledger_id {
                jl.state = TransactionState::Posted;
                jl.posting_ref = line.posting_ref;
            }
        }

        Ok(count as u64)
    }

    async fn update_journal_transaction_line_account_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::line::account::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut dummy = Vec::<journal::transaction::line::account::ActiveModel>::new();
        let mut inner = self.inner.write().await;
        let xact_lines = match inner.journal_xact_line_account.get_mut(&id) {
            Some(lines) => lines,
            None => &mut dummy,
        };
        let count = xact_lines.len();
        for mut jl in xact_lines.iter_mut() {
            if jl.account_id == line.account_id {
                jl.state = TransactionState::Posted;
                jl.posting_ref = line.posting_ref;
            }
        }

        Ok(count as u64)
    }

    async fn find_ledger_by_no(
        &self,
        no: ArrayShortString,
    ) -> Result<Option<ledger::ActiveModel>, OrmError> {
        let inner = self.inner.read().await;
        let list: Vec<(&LedgerId, &ledger::ActiveModel)> = inner
            .ledger
            .iter()
            .filter(|(_, l)| l.ledger_no == no)
            .collect();
        match list.len() {
            0 => return Ok(None),
            1 => {
                let (_, l) = list[0];

                return Ok(Some(*l));
            }
            _ => {
                return Err(OrmError::Validation(
                    "found multiple ledgers with the same ledger number".into(),
                ));
            }
        }
    }

    async fn journal_entries_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        let inner = self.inner.read().await;
        let entries: Vec<ledger::transaction::ActiveModel> = inner
            .journal_entry
            .iter()
            .filter(|(_, value)| ids.contains(&value.ledger_id))
            .map(|(_, am)| *am)
            .collect();

        Ok(entries)
    }

    async fn journal_entry_ledgers_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<ledger::transaction::ledger::ActiveModel>, OrmError> {
        let inner = self.inner.read().await;
        let xacts: Vec<ledger::transaction::ledger::ActiveModel> = inner
            .ledger_xact_ledger
            .iter()
            .filter(|(_, value)| ids.contains(&value.ledger_dr_id))
            .map(|(_, am)| *am)
            .collect();

        Ok(xacts)
    }

    async fn find_journal_by_code<'a>(
        &self,
        _journal_code: &str,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        todo!()
    }

    async fn find_period_by_fiscal_year(
        &self,
        fy: i32,
    ) -> Result<Option<accounting_period::ActiveModel>, OrmError> {
        let inner = self.inner.read().await;
        let list: Vec<(&PeriodId, &accounting_period::ActiveModel)> = inner
            .period
            .iter()
            .filter(|(_, p)| p.fiscal_year == fy)
            .collect();
        match list.len() {
            0 => return Ok(None),
            1 => {
                let (_, p) = list[0];

                return Ok(Some(*p));
            }
            _ => {
                return Err(OrmError::Validation(
                    "found multiple accounting periods with the same fiscal year".into(),
                ))
            }
        }
    }
}
