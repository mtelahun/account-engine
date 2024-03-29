use chrono::{NaiveDate, NaiveDateTime, Utc};
use fake::{faker::name::raw::Name, locales::EN, Fake};
use rand::Rng;
use rust_decimal::Decimal;
use std::str::FromStr;

use account_engine::{
    domain::{
        entity::{
            external_account::account_id::AccountId,
            ledger::ledger_id::LedgerId,
            subsidiary_ledger::{
                external_xact_type_code::ExternalXactTypeCode, subleder_id::SubLedgerId,
            },
        },
        external::{
            EntityTypeBuilder, ExternalAccount, ExternalAccountBuilder, ExternalEntityBuilder,
            ExternalEntityType, ExternalService,
        },
        journal_transaction::{JournalTransactionColumn, SpecialJournalTransaction},
        special_journal::special_journal_template_id::SpecialJournalTemplateId,
        GeneralJournalService, GeneralLedgerService, LedgerAccount, ServiceError,
        SpecialJournalService, SpecialJournalTransactionService, SubsidiaryLedgerService,
    },
    infrastructure::persistence::context::memory::MemoryStore,
    resource::{
        account_engine::AccountEngine,
        external, general_ledger,
        journal::{self, transaction::JournalTransactionColumnType, LedgerAccountPostingRef},
        ledger, subsidiary_ledger, LedgerKey, LedgerType,
    },
    shared_kernel::{
        ArrayString128, ArrayString24, ArrayString3, JournalId, JournalTransactionId, Sequence,
        XactType,
    },
};

use crate::support::{
    service_test_interface::ServiceTestInterface,
    state_interface::StateInterface,
    utils::{random_account_no, timestamp},
    CreateLedgerType, ENTITYTYPE_PERSON,
};

pub async fn state_with_memstore() -> TestState {
    TestState::new().await
}

pub struct TestState {
    pub engine: AccountEngine<MemoryStore>,
    pub general_ledger: general_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
    pub etype_person: ExternalEntityType,
}

impl TestState {
    pub async fn new() -> Self {
        let store = MemoryStore::new_schema("", "")
            .await
            .expect("failed to create MemoryRepository");
        let engine = AccountEngine::new(store)
            .await
            .expect("failed to create engine instance");
        let ledger = general_ledger::Model {
            name: ArrayString128::from_str("My Company").unwrap(),
            currency_code: ArrayString3::from_str("USD").unwrap(),
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
            description: "Default transaction".into(),
        };
        let etype = EntityTypeBuilder::new(ENTITYTYPE_PERSON.into(), "Person".into());
        let etype_person = engine
            .create_entity_type(etype)
            .await
            .expect("failed to initialize external entity type");
        let _ = engine
            .create_external_transaction_type(&model)
            .await
            .expect("failed to create external transaction type");

        Self {
            engine,
            general_ledger: ledger,
            journal,
            etype_person,
        }
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
        let line = journal::transaction::general::line::Model {
            journal_id: journal_id,
            timestamp: now,
            dr_ledger_id: account_dr_id,
            cr_ledger_id: account_cr_id,
            amount: amount,
            ..Default::default()
        };
        let model = journal::transaction::general::Model {
            journal_id,
            timestamp: now,
            explanation: desc.into(),
            lines: vec![line],
        };

        self.engine.create_general_transaction(&model).await
    }

    fn random_account_no() -> String {
        let mut rng = rand::thread_rng();
        let no = rng.gen_range(0..9999);

        format!("{no}")
    }

    pub async fn create_subsidiary_ledger(
        &self,
        name: &'static str,
        account_xact_type: XactType,
    ) -> (
        subsidiary_ledger::ActiveModel,
        journal::ActiveModel,
        LedgerAccount,
        ExternalAccount,
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    ) {
        let control_ledger = self.create_ledger(name, LedgerType::Derived).await;
        let other_ledger = self.create_ledger_leaf().await;
        let sub = subsidiary_ledger::Model {
            name: name.into(),
            ledger_id: control_ledger.id(),
        };
        let sub = self
            .engine
            .create_subsidiary_ledger(&sub)
            .await
            .expect(format!("failed to create subsidiary ledger: {}", sub.name).as_str());
        let account_name: String = Name(EN).fake();
        let account_no = random_account_no();
        let entity =
            ExternalEntityBuilder::new(self.etype_person.code(), account_name.clone().into());
        let entity = self
            .engine
            .create_entity(entity)
            .await
            .expect("failed to create an external entity");
        let acc = ExternalAccountBuilder::new(
            &sub.id,
            &entity.id(),
            account_no.into(),
            account_name.into(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        );
        let acc = self
            .engine
            .create_account(acc)
            .await
            .expect("failed to create external account");
        let account_dr: Option<AccountId>;
        let account_cr: Option<AccountId>;
        if account_xact_type == XactType::Dr {
            account_dr = Some(acc.id());
            account_cr = None;
        } else {
            account_cr = Some(acc.id());
            account_dr = None;
        }
        let (tpl, tpl_column) = self
            .create_special_journal_template(
                account_dr,
                account_cr,
                control_ledger.id(),
                other_ledger.id(),
            )
            .await;
        let journ = journal::Model {
            name: name.into(),
            code: name.into(),
            journal_type: journal::JournalType::Special,
            control_ledger_id: Some(control_ledger.id()),
            template_id: Some(tpl.id),
        };
        let journ = self
            .engine
            .create_journal(&journ)
            .await
            .expect(format!("failed to create journal: {}", journ.name).as_str());

        (sub, journ, control_ledger, acc, tpl, tpl_column)
    }

    pub async fn create_special_transaction(
        &self,
        journal: &journal::ActiveModel,
        tpl_id: &SpecialJournalTemplateId,
        account_id: AccountId,
        account_xact_type: XactType,
        amount: Decimal,
        tpl_col: &Vec<journal::transaction::special::template::column::ActiveModel>,
    ) -> Result<
        (
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        ),
        ServiceError,
    > {
        let timestamp = timestamp();
        let column0: JournalTransactionColumn;
        if account_xact_type == XactType::Dr {
            let column = journal::transaction::column::account_dr::ActiveModel {
                journal_id: journal.id,
                timestamp,
                template_column_id: tpl_col[0].id,
                account_id,
                amount,
                posting_ref: None,
            };
            column0 = JournalTransactionColumn::AccountDr(column);
        } else {
            let column = journal::transaction::column::account_cr::ActiveModel {
                journal_id: journal.id,
                timestamp,
                template_column_id: tpl_col[0].id,
                account_id,
                amount,
                posting_ref: None,
            };
            column0 = JournalTransactionColumn::AccountCr(column);
        }
        let column1 = journal::transaction::column::ledger_drcr::ActiveModel {
            journal_id: journal.id,
            timestamp,
            template_column_id: tpl_col[1].id,
            amount: amount,
            ledger_dr_id: tpl_col[1].dr_ledger_id.unwrap(),
            ledger_cr_id: tpl_col[1].cr_ledger_id.unwrap(),
            column_total_id: None,
        };
        let column1 = JournalTransactionColumn::LedgerDrCr(column1);

        SpecialJournalService::create_special_transaction(
            &self.engine,
            journal.id,
            timestamp,
            "Sale of widget".into(),
            *tpl_id,
            "DF".into(),
            &[column0, column1],
        )
        .await
    }

    async fn create_special_journal_template(
        &self,
        account_dr_id: Option<AccountId>,
        account_cr_id: Option<AccountId>,
        ledger_dr_id: LedgerId,
        ledger_cr_id: LedgerId,
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
        let column_type: JournalTransactionColumnType;
        if account_dr_id.is_some() {
            column_type = JournalTransactionColumnType::AccountDr;
        } else {
            column_type = JournalTransactionColumnType::AccountCr;
        }
        let act_column = journal::transaction::special::template::column::Model {
            template_id: template.id,
            sequence: Sequence::new(1).unwrap(),
            name: "Account".into(),
            column_type,
            cr_ledger_id: None,
            dr_ledger_id: None,
            dr_account_id: account_dr_id,
            cr_account_id: account_cr_id,
        };
        let ldgr_column = journal::transaction::special::template::column::Model {
            template_id: template.id,
            sequence: Sequence::new(2).unwrap(),
            name: "Accounts Receivable / Revenue".into(),
            column_type: JournalTransactionColumnType::LedgerDrCr,
            cr_ledger_id: Some(ledger_cr_id),
            dr_ledger_id: Some(ledger_dr_id),
            dr_account_id: None,
            cr_account_id: None,
        };

        let columns = self
            .engine
            .create_journal_template_columns(vec![&act_column, &ldgr_column])
            .await
            .expect("failed to create sales journal template");

        (template, columns)
    }
}

#[async_trait::async_trait]
impl StateInterface for TestState {
    async fn create_account(
        &self,
        number: &str,
        name: &'static str,
        subledger_id: SubLedgerId,
    ) -> ExternalAccount {
        let entity = ExternalEntityBuilder::new(self.etype_person.code(), name.into());
        let entity = self
            .engine
            .create_entity(entity)
            .await
            .expect("failed to create an external entity");
        let account = ExternalAccountBuilder::new(
            &subledger_id,
            &entity.id(),
            ArrayString24::from(number),
            ArrayString128::from(name),
            Utc::now().date_naive(),
        );
        self.engine
            .create_account(account)
            .await
            .expect("failed to create external account")
    }

    async fn create_ledger(&self, name: &'static str, typ: LedgerType) -> LedgerAccount {
        self.engine
            .create_ledger(
                typ,
                self.general_ledger.root,
                name,
                &Self::random_account_no(),
                None,
            )
            .await
            .expect("failed to create a ledger")
    }

    async fn create_ledger_leaf(&self) -> LedgerAccount {
        self.create_ledger("Some Ledger", LedgerType::Leaf).await
    }

    async fn create_subsidiary(
        &self,
        name: &'static str,
        control: CreateLedgerType,
    ) -> (
        subsidiary_ledger::ActiveModel,
        journal::ActiveModel,
        LedgerAccount,
        ExternalAccount,
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    ) {
        let control_ledger = match control {
            CreateLedgerType::Ledger(l) => l,
            _ => self.create_ledger(name, LedgerType::Derived).await,
        };
        let other_ledger = self.create_ledger_leaf().await;
        let sub = subsidiary_ledger::Model {
            name: name.into(),
            ledger_id: control_ledger.id(),
        };
        let sub = self
            .engine
            .create_subsidiary_ledger(&sub)
            .await
            .expect(format!("failed to create subsidiary ledger: {}", sub.name).as_str());
        let (tpl, tpl_column) = self
            .create_template_journal(control_ledger.id(), other_ledger.id())
            .await;
        let journ = journal::Model {
            name: name.into(),
            code: name.into(),
            journal_type: journal::JournalType::Special,
            control_ledger_id: Some(control_ledger.id()),
            template_id: Some(tpl.id),
        };
        let journ = self
            .engine
            .create_journal(&journ)
            .await
            .expect(format!("failed to create journal: {}", journ.name).as_str());
        let name: String = Name(EN).fake();
        let entity = ExternalEntityBuilder::new(self.etype_person.code(), name.clone().into());
        let entity = self
            .engine
            .create_entity(entity)
            .await
            .expect("failed to create an external entity");
        let acc = ExternalAccountBuilder::new(
            &sub.id,
            &entity.id(),
            random_account_no().into(),
            name.into(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        );
        let acc = self
            .engine
            .create_account(acc)
            .await
            .expect("failed to create external account");

        (sub, journ, control_ledger, acc, tpl, tpl_column)
    }

    async fn create_journal(
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

    async fn create_journal_xact(
        &self,
        _amount: Decimal,
        _account_dr_id: LedgerId,
        _account_cr_id: LedgerId,
        _desc: &str,
        _journal_id: Option<JournalId>,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        todo!()
    }

    async fn create_template_journal(
        &self,
        ledger_dr_id: LedgerId,
        ledger_cr_id: LedgerId,
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
        let column = journal::transaction::special::template::column::Model {
            template_id: template.id,
            sequence: Sequence::new(1).unwrap(),
            name: "Accounts Receivable / Revenue".into(),
            column_type: JournalTransactionColumnType::LedgerDrCr,
            cr_ledger_id: Some(ledger_cr_id),
            dr_ledger_id: Some(ledger_dr_id),
            dr_account_id: None,
            cr_account_id: None,
        };

        let columns = self
            .engine
            .create_journal_template_columns(vec![&column])
            .await
            .expect("failed to create sales journal template");

        (template, columns)
    }
}

#[async_trait::async_trait]
impl ServiceTestInterface for TestState {
    async fn journal_entry_by_key(
        &self,
        _key: LedgerKey,
    ) -> Result<Option<ledger::transaction::ActiveModel>, ServiceError> {
        todo!()
    }

    async fn get_journal_entry_transaction_account(
        &self,
        _posting_ref: &LedgerAccountPostingRef,
    ) -> Result<ledger::transaction::account::ActiveModel, ServiceError> {
        todo!()
    }

    async fn create_general_transaction(
        &self,
        _model: &journal::transaction::general::Model,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        todo!()
    }

    async fn create_subsidiary_transaction<'a>(
        &self,
        journal_id: &JournalId,
        timestamp: NaiveDateTime,
        tpl_id: &SpecialJournalTemplateId,
        _account_id: AccountId,
        _account_xact_type: XactType,
        xact_type_external_code: &ExternalXactTypeCode,
        _amount: Decimal,
        explanation: &ArrayString128,
        _tpl_col: &Vec<journal::transaction::special::template::column::ActiveModel>,
        line_models: &'a [JournalTransactionColumn],
    ) -> Result<
        (
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        ),
        ServiceError,
    > {
        self.engine
            .create_special_transaction(
                *journal_id,
                timestamp,
                *explanation,
                *tpl_id,
                *xact_type_external_code,
                line_models,
            )
            .await
    }

    async fn get_subsidiary_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    > {
        self.engine.get_special_transactions(ids).await
    }

    async fn get_subsidiary_transactions_by_journal(
        &self,
        id: JournalId,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    > {
        self.engine.get_subsidiary_transactions_by_journal(id).await
    }

    async fn get_subsidiary_transaction_columns(
        &self,
        id: JournalTransactionId,
    ) -> Result<Vec<JournalTransactionColumn>, ServiceError> {
        self.engine.get_special_transaction_columns(id).await
    }

    async fn post_transaction(&self, _id: JournalTransactionId) -> Result<bool, ServiceError> {
        todo!()
    }

    async fn post_subsidiary_ledger(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        self.engine.post_to_account(id).await
    }

    async fn post_general_ledger(
        &self,
        journal_id: JournalId,
        ids: &Vec<JournalTransactionId>,
    ) -> Result<bool, ServiceError> {
        self.engine.post_general_ledger(journal_id, ids).await
    }

    async fn get_column_total(
        &self,
        _id: JournalTransactionId,
        _sequence: Sequence,
    ) -> Result<journal::transaction::special::column::sum::ActiveModel, ServiceError> {
        todo!()
    }
}
