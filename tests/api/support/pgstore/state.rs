use std::str::FromStr;

use account_engine::{
    domain::{
        external::{
            EntityTypeBuilder, ExternalAccount, ExternalAccountBuilder, ExternalEntityBuilder,
            ExternalEntityType, ExternalService,
        },
        general_ledger::ledger_id::LedgerId,
        journal_transaction::{JournalTransactionColumn, SpecialJournalTransaction},
        special_journal::special_journal_template_id::SpecialJournalTemplateId,
        subsidiary_ledger::account_id::AccountId,
        GeneralJournalService, GeneralLedgerService, LedgerAccount, ServiceError,
        SpecialJournalService, SubsidiaryLedgerService,
    },
    infrastructure::data::db_context::postgres::PostgresStore,
    resource::{
        account_engine::AccountEngine,
        external, general_ledger,
        journal::{self, transaction::JournalTransactionColumnType},
        subsidiary_ledger, LedgerType,
    },
    shared_kernel::{ArrayString128, ArrayString3, JournalId, Sequence, XactType},
};
use chrono::NaiveDate;
use fake::{faker::name::en::Name, Fake};
use rust_decimal::Decimal;

use crate::{
    pg_store::SimpleJournalTransaction,
    support::{
        utils::{random_account_no, timestamp},
        ENTITYTYPE_PERSON,
    },
};

pub struct TestState {
    pub db_name: String,
    pub engine: AccountEngine<PostgresStore>,
    pub general_ledger: general_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
    pub etype_person: ExternalEntityType,
}

impl TestState {
    pub async fn new() -> TestState {
        let db_name = uuid::Uuid::new_v4().to_string();
        let postgres_url = format!("postgres://postgres:password@localhost:5432");
        let store = PostgresStore::new_schema(&db_name, &postgres_url)
            .await
            .expect("failed to connect to newly created database: {db_name}");
        let engine = AccountEngine::new(store)
            .await
            .expect("failed to instantiate account-engine");
        println!("Database name: {db_name}");

        let general_ledger = general_ledger::Model {
            name: ArrayString128::from_str("My Compnay").unwrap(),
            currency_code: ArrayString3::from_str("USD").unwrap(),
        };
        let general_ledger = GeneralLedgerService::update_general_ledger(&engine, &general_ledger)
            .await
            .expect("failed to update general ledger");
        let journal = Self::create_general_journal(&engine).await;
        let etype = EntityTypeBuilder::new(ENTITYTYPE_PERSON.into(), "Person".into());
        let etype_person = engine
            .create_entity_type(etype)
            .await
            .expect("failed to initialize external entity type");
        let model = external::transaction_type::Model {
            code: "DF".into(),
            description: "Default transaction".into(),
        };
        let _ = engine
            .create_external_transaction_type(&model)
            .await
            .expect("failed to create external transaction type");

        Self {
            engine,
            general_ledger,
            journal,
            db_name,
            etype_person,
        }
    }

    async fn create_ledger(&self, name: &'static str, typ: LedgerType) -> LedgerAccount {
        self.engine
            .create_ledger(
                typ,
                self.general_ledger.root,
                name,
                &random_account_no(),
                None,
            )
            .await
            .expect("failed to create a ledger")
    }

    async fn create_ledger_leaf(&self) -> LedgerAccount {
        self.create_ledger("Some Ledger", LedgerType::Leaf).await
    }

    pub async fn create_account(
        &self,
        number: &'static str,
        name: &'static str,
        typ: LedgerType,
        parent_id: LedgerId,
    ) -> Result<LedgerAccount, ServiceError> {
        GeneralLedgerService::create_ledger(&self.engine, typ, parent_id, name, number, None).await
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

        GeneralLedgerService::create_journal(&self.engine, &model).await
    }

    pub async fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: LedgerId,
        account_cr_id: LedgerId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        let timestamp = timestamp();
        let journal_id: JournalId = journal_id.unwrap_or(self.journal.id);
        let line1 = journal::transaction::general::line::Model {
            journal_id: journal_id,
            timestamp,
            dr_ledger_id: account_dr_id,
            cr_ledger_id: account_cr_id,
            amount,
            ..Default::default()
        };
        let model = journal::transaction::general::Model {
            journal_id,
            timestamp,
            explanation: ArrayString128::from(desc),
            lines: vec![line1],
        };

        self.engine.create_general_transaction(&model).await
    }

    pub fn simple_xact_model(&self) -> SimpleJournalTransaction {
        let timestamp = timestamp();
        let line1 = journal::transaction::general::line::Model {
            journal_id: self.journal.id,
            timestamp,
            amount: Decimal::ZERO,
            ..Default::default()
        };
        let jx = journal::transaction::general::Model {
            journal_id: self.journal.id,
            timestamp,
            explanation: "Withdrew cash for lunch".into(),
            lines: Vec::<journal::transaction::general::line::Model>::new(),
        };

        SimpleJournalTransaction {
            jx,
            line1,
            timestamp: timestamp,
        }
    }

    async fn create_general_journal(engine: &AccountEngine<PostgresStore>) -> journal::ActiveModel {
        let journal = journal::Model {
            name: "General Journal".into(),
            code: "J".into(),
            ..Default::default()
        };
        let journal = engine.create_journal(&journal).await.unwrap();

        journal
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
        let account_name: String = Name().fake();
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
