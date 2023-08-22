use std::str::FromStr;

use account_engine::{
    domain::{ArrayCodeString, ArrayLongString, ArrayShortString, JournalId, LedgerId, XactType},
    resource::{
        account_engine::AccountEngine, external, general_ledger, journal, ledger,
        subsidiary_ledger, LedgerType,
    },
    service::{
        GeneralJournalService, GeneralLedgerService, ServiceError, SubsidiaryJournalService,
        SubsidiaryLedgerService,
    },
    store::memory::store::MemoryStore,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;

use crate::{
    support::utils::{random_account_name, random_account_no, timestamp},
    CreateLedgerType,
};

pub struct TestState {
    pub engine: AccountEngine<MemoryStore>,
    pub general_ledger: general_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
}

impl TestState {
    pub async fn new() -> TestState {
        let store = MemoryStore::new_schema("", "")
            .await
            .expect("failed to create MemoryRepository");
        let engine = AccountEngine::new(store)
            .await
            .expect("failed to create engine instance");
        let ledger = general_ledger::Model {
            name: ArrayLongString::from_str("My Company").unwrap(),
            currency_code: ArrayCodeString::from_str("USD").unwrap(),
        };
        let ledger = engine
            .update_general_ledger(&ledger)
            .await
            .expect("failed to update general ledger");
        let journal = journal::Model {
            name: "General Journal".into(),
            code: "G".into(),
            ..Default::default()
        };
        let journal = engine
            .create_journal(&journal)
            .await
            .expect("failed to create main journal");
        let model = external::transaction_type::Model {
            code: "DF".into(),
            entity_type_code: "NO".into(),
            description: "Default transaction".into(),
        };
        let _ = engine
            .create_external_transaction_type(&model)
            .await
            .expect("failed to create external transaction type");

        Self {
            engine,
            general_ledger: ledger,
            journal,
        }
    }

    pub async fn create_account(
        &self,
        number: &str,
        name: &'static str,
        typ: LedgerType,
        parent_id: Option<LedgerId>,
    ) -> Result<ledger::ActiveModel, ServiceError> {
        let account = ledger::Model {
            ledger_no: ArrayShortString::from_str(number).unwrap(),
            ledger_type: typ,
            parent_id,
            name: ArrayLongString::from_str(name).unwrap(),
            currency_code: None,
        };

        self.engine.create_ledger(&account).await
    }

    pub async fn create_ledger(&self, name: &'static str, typ: LedgerType) -> ledger::ActiveModel {
        let ledger = ledger::Model {
            ledger_no: random_account_no().into(),
            ledger_type: typ,
            parent_id: Some(self.general_ledger.root),
            name: name.into(),
            currency_code: None,
        };

        self.engine
            .create_ledger(&ledger)
            .await
            .expect("failed to create ledger")
    }

    pub async fn create_ledger_leaf(&self) -> ledger::ActiveModel {
        let ledger = ledger::Model {
            ledger_no: random_account_no().into(),
            ledger_type: LedgerType::Leaf,
            parent_id: Some(self.general_ledger.root),
            name: random_account_name().into(),
            currency_code: None,
        };

        self.engine
            .create_ledger(&ledger)
            .await
            .expect("failed to create ledger")
    }

    pub async fn create_subsidiary(
        &self,
        name: &'static str,
        control: CreateLedgerType,
    ) -> (
        subsidiary_ledger::ActiveModel,
        journal::ActiveModel,
        ledger::ActiveModel,
        external::account::ActiveModel,
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    ) {
        let ledg = match control {
            CreateLedgerType::Ledger(l) => l,
            _ => self.create_ledger(name, LedgerType::Derived).await,
        };
        let sub = subsidiary_ledger::Model {
            name: name.into(),
            ledger_account_id: ledg.id,
        };
        let sub = self
            .engine
            .create_subsidiary_ledger(&sub)
            .await
            .expect(format!("failed to create subsidiary ledger: {}", sub.name).as_str());
        let (tpl, tpl_column) = self.create_template_journal().await;
        let journ = journal::Model {
            name: name.into(),
            code: name.into(),
            journal_type: journal::JournalType::Special,
            ledger_id: Some(ledg.id),
            template_id: Some(tpl.id),
        };
        let journ = self
            .engine
            .create_journal(&journ)
            .await
            .expect(format!("failed to create journal: {}", journ.name).as_str());
        let acc = external::account::Model {
            subledger_id: sub.id,
            entity_type_code: "PE".into(),
            account_no: random_account_no().into(),
            date_opened: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        };
        let acc = self
            .engine
            .create_account(&acc)
            .await
            .expect("failed to create external account");

        (sub, journ, ledg, acc, tpl, tpl_column)
    }

    async fn create_template_journal(
        &self,
    ) -> (
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    ) {
        let sales = journal::transaction::special::template::Model {
            name: "Sales".into(),
        };
        let template = self
            .engine
            .create_journal_template(&sales)
            .await
            .expect("failed to create sales journal template");
        let ledger_sales = self.create_ledger("Sales", LedgerType::Derived).await;
        let ledger_ar = self
            .create_ledger("Acounts Receivable", LedgerType::Derived)
            .await;
        let column = journal::transaction::special::template::column::Model {
            template_id: template.id,
            sequence: 1,
            cr_ledger_id: Some(ledger_sales.id),
            dr_ledger_id: Some(ledger_ar.id),
        };

        let columns = self
            .engine
            .create_journal_template_columns(vec![&column])
            .await
            .expect("failed to create sales journal template");

        (template, columns)
    }

    pub async fn create_journal(
        &self,
        code: &'static str,
        name: &'static str,
    ) -> Result<journal::ActiveModel, ServiceError> {
        let model = journal::Model {
            name: name.into(),
            code: code.into(),
            ..Default::default()
        };

        self.engine.create_journal(&model).await
    }

    pub async fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: LedgerId,
        account_cr_id: LedgerId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        let now = timestamp();
        let journal_id: JournalId = journal_id.unwrap_or(self.journal.id);
        let dr_line = journal::transaction::general::line::Model {
            journal_id: journal_id,
            timestamp: now,
            ledger_id: account_dr_id,
            xact_type: XactType::Dr,
            amount: amount,
            ..Default::default()
        };
        let cr_line = journal::transaction::general::line::Model {
            journal_id: journal_id,
            timestamp: now,
            ledger_id: account_cr_id,
            xact_type: XactType::Cr,
            amount: amount,
            ..Default::default()
        };
        let model = journal::transaction::general::Model {
            journal_id,
            timestamp: now,
            explanation: desc.into(),
            lines: vec![dr_line, cr_line],
        };

        self.engine.create_general_transaction(&model).await
    }
}
