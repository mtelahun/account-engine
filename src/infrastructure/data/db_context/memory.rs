use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    domain::{
        general_ledger::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId},
        period::{interim_period::InterimPeriodId, period_id::PeriodId},
        special_journal::{
            column_total_id::ColumnTotalId, special_journal_template_id::SpecialJournalTemplateId,
            template_column_id::TemplateColumnId,
        },
        subsidiary_ledger::{
            account_id::AccountId, entity_code::EntityCode,
            external_xact_type_code::ExternalXactTypeCode, subleder_id::SubLedgerId,
        },
    },
    resource::{
        accounting_period, external, general_ledger,
        journal::{self, transaction::TransactionState},
        ledger::{self, journal_entry::LedgerKey, LedgerType},
        ledger_xact_type, organization, subsidiary_ledger,
    },
    shared_kernel::{
        ids::ExternalEntityId, journal_transaction_column_id::JournalTransactionColumnId,
        AccountTransactionId, ArrayString128, ArrayString24, ArrayString3, JournalId,
        JournalTransactionId, LedgerXactTypeCode, Sequence,
    },
    Store,
};

use super::error::OrmError;

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
    pub(crate) ledger_derived: HashMap<LedgerId, ledger::derived::ActiveModel>,
    pub(crate) ledger_intermediate: HashMap<LedgerId, ledger::intermediate::ActiveModel>,
    pub(crate) ledger_leaf: HashMap<LedgerId, ledger::leaf::ActiveModel>,
    pub(crate) period: HashMap<PeriodId, accounting_period::ActiveModel>,
    pub(crate) interim_period:
        HashMap<InterimPeriodId, accounting_period::interim_period::ActiveModel>,
    pub(crate) journal: HashMap<JournalId, journal::ActiveModel>,
    pub(crate) journal_xact: HashMap<JournalTransactionId, journal::transaction::ActiveModel>,
    pub(crate) journal_xact_record_sub:
        HashMap<JournalTransactionId, journal::transaction::special::ActiveModel>,
    pub(crate) journal_xact_general:
        HashMap<JournalTransactionId, Vec<journal::transaction::general::line::ActiveModel>>,
    pub(crate) journal_xact_special_totals:
        HashMap<JournalTransactionId, journal::transaction::special::summary::ActiveModel>,
    pub(crate) journal_xact_special_column:
        HashMap<JournalTransactionId, Vec<journal::transaction::special::column::ActiveModel>>,
    pub(crate) journal_xact_column_ledger_drcr: HashMap<
        JournalTransactionColumnId,
        Vec<journal::transaction::column::ledger_drcr::ActiveModel>,
    >,
    pub(crate) journal_xact_column_text:
        HashMap<JournalTransactionColumnId, Vec<journal::transaction::column::text::ActiveModel>>,
    pub(crate) journal_xact_column_account_dr:
        HashMap<JournalTransactionColumnId, journal::transaction::column::account_dr::ActiveModel>,
    pub(crate) journal_xact_column_account_cr:
        HashMap<JournalTransactionColumnId, journal::transaction::column::account_cr::ActiveModel>,
    pub(crate) journal_xact_special_colum_total:
        HashMap<ColumnTotalId, journal::transaction::special::column::sum::ActiveModel>,
    pub(crate) journal_xact_sub_template:
        HashMap<SpecialJournalTemplateId, journal::transaction::special::template::ActiveModel>,
    pub(crate) journal_xact_sub_template_column:
        HashMap<TemplateColumnId, journal::transaction::special::template::column::ActiveModel>,
    pub(crate) journal_entry: HashMap<LedgerKey, ledger::transaction::ActiveModel>,
    pub(crate) ledger_xact_account: HashMap<LedgerKey, ledger::transaction::account::ActiveModel>,
    pub(crate) ledger_xact_ledger: HashMap<LedgerKey, ledger::transaction::ledger::ActiveModel>,
    pub(crate) ledger_xact_type: HashMap<LedgerXactTypeCode, ledger_xact_type::ActiveModel>,
    pub(crate) external_xact_type:
        HashMap<ExternalXactTypeCode, external::transaction_type::ActiveModel>,
    pub(crate) subsidiary_ledger: HashMap<SubLedgerId, subsidiary_ledger::ActiveModel>,
    pub(crate) external_account: HashMap<AccountId, external::account::ActiveModel>,
    pub(crate) external_account_transaction:
        HashMap<AccountTransactionId, external::account::transaction::ActiveModel>,
    pub(crate) external_entity: HashMap<ExternalEntityId, external::entity::ActiveModel>,
    pub(crate) entity_type: HashMap<EntityCode, external::entity_type::ActiveModel>,
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
            ledger_derived: HashMap::<LedgerId, ledger::derived::ActiveModel>::new(),
            ledger_intermediate: HashMap::<LedgerId, ledger::intermediate::ActiveModel>::new(),
            ledger_leaf: HashMap::<LedgerId, ledger::leaf::ActiveModel>::new(),
            period: HashMap::<PeriodId, accounting_period::ActiveModel>::new(),
            interim_period: HashMap::<
                InterimPeriodId,
                accounting_period::interim_period::ActiveModel,
            >::new(),
            journal: HashMap::<JournalId, journal::ActiveModel>::new(),
            journal_xact: HashMap::<JournalTransactionId, journal::transaction::ActiveModel>::new(),
            journal_xact_record_sub: HashMap::<
                JournalTransactionId,
                journal::transaction::special::ActiveModel,
            >::new(),
            journal_xact_general: HashMap::<
                JournalTransactionId,
                Vec<journal::transaction::general::line::ActiveModel>,
            >::new(),
            journal_xact_special_totals: HashMap::<
                JournalTransactionId,
                journal::transaction::special::summary::ActiveModel,
            >::new(),
            journal_xact_special_column: HashMap::<
                JournalTransactionId,
                Vec<journal::transaction::special::column::ActiveModel>,
            >::new(),
            journal_xact_column_ledger_drcr: HashMap::<
                JournalTransactionColumnId,
                Vec<journal::transaction::column::ledger_drcr::ActiveModel>,
            >::new(),
            journal_xact_column_text: HashMap::<
                JournalTransactionColumnId,
                Vec<journal::transaction::column::text::ActiveModel>,
            >::new(),
            journal_xact_column_account_dr: HashMap::<
                JournalTransactionColumnId,
                journal::transaction::column::account_dr::ActiveModel,
            >::new(),
            journal_xact_column_account_cr: HashMap::<
                JournalTransactionColumnId,
                journal::transaction::column::account_cr::ActiveModel,
            >::new(),
            journal_xact_special_colum_total: HashMap::<
                ColumnTotalId,
                journal::transaction::special::column::sum::ActiveModel,
            >::new(),
            journal_xact_sub_template: HashMap::<
                SpecialJournalTemplateId,
                journal::transaction::special::template::ActiveModel,
            >::new(),
            journal_xact_sub_template_column: HashMap::<
                TemplateColumnId,
                journal::transaction::special::template::column::ActiveModel,
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
            subsidiary_ledger: HashMap::<SubLedgerId, subsidiary_ledger::ActiveModel>::new(),
            external_account: HashMap::<AccountId, external::account::ActiveModel>::new(),
            external_account_transaction: HashMap::<
                AccountTransactionId,
                external::account::transaction::ActiveModel,
            >::new(),
            external_entity: HashMap::<ExternalEntityId, external::entity::ActiveModel>::new(),
            entity_type: HashMap::<EntityCode, external::entity_type::ActiveModel>::new(),
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

    pub async fn new_schema(name: &str, uri: &str) -> Result<Self, OrmError> {
        Ok(Self::new(name, uri))
    }

    fn insert_ledger_root(&mut self) -> LedgerId {
        let root_id = LedgerId::new();
        let root = ledger::ActiveModel {
            id: root_id,
            number: ArrayString24::from_str("0").unwrap(),
            ledger_type: LedgerType::Intermediate,
            parent_id: None,
            name: ArrayString128::from_str("Root").unwrap(),
            currency_code: None,
        };
        self.ledger.insert(root_id, root);
        self.ledger_intermediate
            .insert(root_id, ledger::intermediate::ActiveModel { id: root_id });

        root_id
    }

    fn insert_general_leger(&mut self, root_id: LedgerId) {
        let id = GeneralLedgerId::new();
        let v = general_ledger::ActiveModel {
            id,
            name: ArrayString128::from_str("Root").unwrap(),
            currency_code: ArrayString3::from_str("USD").unwrap(),
            root: root_id,
        };
        self.general_ledger.insert(id, v);
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
        line: &journal::transaction::general::line::ActiveModel,
    ) -> Result<u64, OrmError> {
        let mut dummy = Vec::<journal::transaction::general::line::ActiveModel>::new();
        let mut inner = self.inner.write().await;
        let xact_lines = match inner.journal_xact_general.get_mut(&id) {
            Some(lines) => lines,
            None => &mut dummy,
        };
        let count = xact_lines.len();
        for jl in xact_lines.iter_mut() {
            if jl.dr_ledger_id == line.dr_ledger_id {
                jl.state = TransactionState::Posted;
                jl.dr_posting_ref = line.dr_posting_ref;
                jl.cr_posting_ref = line.cr_posting_ref;
            }
        }

        Ok(count as u64)
    }

    async fn find_ledger_by_no(
        &self,
        no: ArrayString24,
    ) -> Result<Option<ledger::ActiveModel>, OrmError> {
        let inner = self.inner.read().await;
        let list: Vec<(&LedgerId, &ledger::ActiveModel)> = inner
            .ledger
            .iter()
            .filter(|(_, l)| l.number == no)
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

    async fn get_journal_transaction_columns<'a>(
        &self,
        ids: &'a [JournalTransactionId],
        sequence: Sequence,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        let mut result = Vec::<journal::transaction::special::column::ActiveModel>::new();
        let inner = self.inner.read().await;
        for id in ids.iter() {
            let value = inner.journal_xact_special_column.get(id);
            if value.is_none() {
                continue;
            }
            for column in value.unwrap().iter() {
                if column.sequence == sequence {
                    result.push(*column);
                }
            }
        }

        Ok(result)
    }

    async fn get_journal_transaction_template_columns(
        &self,
        id: SpecialJournalTemplateId,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        let mut result = Vec::<journal::transaction::special::template::column::ActiveModel>::new();
        let inner = self.inner.read().await;
        for (_, value) in inner.journal_xact_sub_template_column.iter() {
            if value.template_id == id {
                result.push(*value);
            }
        }

        Ok(result)
    }
}
