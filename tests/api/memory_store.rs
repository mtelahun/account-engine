use std::str::FromStr;

use account_engine::{
    domain::{
        ids::JournalId, AccountId, ArrayCodeString, ArrayLongString, ArrayShortString,
        JournalTransactionId, XactType,
    },
    resource::{
        account_engine::AccountEngine, accounting_period, external, general_ledger, journal,
        ledger, subsidiary_ledger, InterimType, LedgerType, TransactionState,
    },
    service::{
        AccountingPeriodService, GeneralLedgerService, JournalService, JournalTransactionService,
        LedgerService, ServiceError, SubsidiaryLedgerService,
    },
    store::{memory::store::MemoryStore, OrmError},
};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::{timestamp, utils::random_account_no};

#[tokio::test]
async fn non_existant_ledger() {
    // Arrange
    let state = TestState::new().await;
    let ledger_id = AccountId::new();

    // Act
    let res = state
        .engine
        .get_ledgers(Some(&vec![ledger_id]))
        .await
        .unwrap();

    // Assert
    let all: Vec<ledger::ActiveModel> = GeneralLedgerService::get_ledgers(&state.engine, None)
        .await
        .unwrap();
    assert_eq!(all.len(), 1, "0 ledgers (minus root) in the list");
    assert_eq!(
        res.len(),
        0,
        "search for non-existent ledger returns nothing"
    );
}

#[tokio::test]
async fn unique_account_number() {
    // Arrange
    let state = TestState::new().await;

    // Act
    let _ = state
        .create_account(
            "1000",
            "Assets",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("1st ledger creation failed");
    let assets_same_gl = state
        .create_account(
            "1000",
            "Assets",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await;

    // Assert
    assert!(assets_same_gl.is_err(), "failed to create second account");
    assert_eq!(
        assets_same_gl.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Validation(
            "duplicate ledger number: 1000".into(),
        ))
        .err()
        .unwrap()
    );
    let gl1_accounts = state.engine.get_ledgers(None).await.unwrap();
    assert_eq!(
        gl1_accounts.len(),
        2,
        "Only one account (+ root) in the 1st ledger"
    );
}

#[tokio::test]
async fn account_parent_is_not_null() {
    // Arrange
    let state = TestState::new().await;

    // Act
    let _ = state
        .create_account(
            "1000",
            "Assets",
            LedgerType::Intermediate,
            Some(state.general_ledger.root),
        )
        .await
        .expect("Assets intermediate account creation failed");
    let cash = state
        .create_account("1001", "Cash", LedgerType::Leaf, None)
        .await;

    // Assert
    assert!(cash.is_err(), "ledger creation w/out parent_id must fail");
    assert_eq!(
        cash.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Validation("ledger must have parent".into(),))
            .err()
            .unwrap()
    );
}

#[tokio::test]
async fn parent_is_intermediate() {
    // Arrange
    let state = TestState::new().await;

    // Act
    let assets = state
        .create_account(
            "1000",
            "Assets",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .unwrap();
    let cash = state
        .create_account("1001", "Cash", LedgerType::Leaf, Some(assets.id))
        .await;

    // Assert
    assert!(cash.is_err(), "failed to create cash account");
    assert_eq!(
        cash.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Validation(
            "parent ledger is not an Intermediate Ledger".into(),
        ))
        .err()
        .unwrap()
    );
}

#[tokio::test]
async fn duplicate_account_name_ok() {
    // Arrange
    let state = TestState::new().await;
    let assets_original = state
        .create_account(
            "1000",
            "Assets",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("first account created successfully");

    // Act
    let assets_same_gl = state
        .create_account(
            "1001",
            "Assets",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("second account with same name created successfully");

    // Assert
    assert_eq!(
        assets_original.name, assets_same_gl.name,
        "account with duplicate name created successfully"
    );
    let gl1_accounts = state
        .engine
        .get_ledgers(None)
        .await
        .expect("unexpected search error");
    assert_eq!(
        gl1_accounts.len(),
        3,
        "Both accounts (+ root) appear in the ledger"
    );
}

#[tokio::test]
async fn unique_accounting_period() {
    // Arrance
    let state = TestState::new().await;
    let period = accounting_period::Model {
        fiscal_year: 2023,
        period_start: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        period_type: InterimType::CalendarMonth,
    };

    // Act
    let _ = state
        .engine
        .create_period(&period)
        .await
        .expect("1st fiscal year creation failed");
    let fy_duplicate = state.engine.create_period(&period).await;

    // Assert
    assert!(
        fy_duplicate.is_err(),
        "duplicate fiscal year creation must fail"
    );
    assert_eq!(
        fy_duplicate.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Validation(
            "duplicate accounting period".into()
        ))
        .err()
        .unwrap()
    );
    let periods = state
        .engine
        .get_periods(None)
        .await
        .expect("unexpected search error");
    assert_eq!(periods.len(), 1, "Only one period in the list")
}

#[tokio::test]
async fn create_accounting_period_calendar() {
    // Arrange
    let state = TestState::new().await;
    let fy = accounting_period::Model {
        fiscal_year: 2023,
        period_start: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        period_type: InterimType::CalendarMonth,
    };
    let dates = vec![
        (
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 1, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 2, 28).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 3, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 4, 30).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 5, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 30).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 7, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 8, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 9, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 10, 31).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 11, 30).unwrap(),
        ),
        (
            NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
            NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        ),
    ];
    let fy = state.engine.create_period(&fy).await.unwrap();

    // Act
    let subperiods: Vec<accounting_period::interim_period::ActiveModel> = state
        .engine
        .get_interim_periods(None)
        .await
        .expect("unexpected search error");

    // Assert
    assert_eq!(subperiods.len(), 12, "12 periods in Calendar Month period");
    let mut idx = 0;
    for interim in subperiods {
        let (start, end) = dates[idx];
        assert_eq!(
            interim.start, start,
            "Start of interim period {} matches",
            idx
        );
        assert_eq!(interim.end, end, "End of interim period {} matches", idx);
        idx += 1;
    }
    assert_eq!(
        fy.period_start,
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        "period start is Jan 1, 2023"
    );
    assert_eq!(
        fy.period_end,
        NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        "period end is Dec 31, 2023"
    );
}

#[tokio::test]
async fn unique_journal_name() {
    // Arrange
    let state = TestState::new().await;
    let j1 = journal::Model {
        name: "General".into(),
        code: "S".into(),
    };
    let j2 = journal::Model {
        name: "Sales".into(),
        code: "S".into(),
    };

    // Act
    let _ = state
        .engine
        .create_journal(&j1)
        .await
        .expect("failed to create first jounral");
    let journal2 = state.engine.create_journal(&j2).await;

    // Assert
    assert!(journal2.is_err(), "failed to create second ledger");
    assert_eq!(
        journal2.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Resource(OrmError::Internal(
            "duplicate Journal Id or Code".into()
        )))
        .err()
        .unwrap()
    );
    let journals = state
        .engine
        .get_journals(None)
        .await
        .expect("unexpected search error");
    assert_eq!(
        journals.len(),
        2,
        "One (+1 by the test harness) journals created"
    );
}

#[tokio::test]
async fn journal_transaction_creation() {
    // Arrange
    let state = TestState::new().await;
    let cash1 = state
        .create_account(
            "1001",
            "Cash",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .unwrap();
    let bank1 = state
        .create_account(
            "1002",
            "Bank",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .unwrap();

    let now = timestamp();
    let jx1_line1 = journal::transaction::line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        ledger_id: Some(cash1.id),
        xact_type: XactType::Dr,
        amount: Decimal::ZERO,
        ..Default::default()
    };
    let jx1_line2 = journal::transaction::line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        ledger_id: Some(bank1.id),
        xact_type: XactType::Cr,
        amount: Decimal::ZERO,
        ..Default::default()
    };
    let jx1 = journal::transaction::Model {
        journal_id: state.journal.id,
        timestamp: now,
        explanation: "Withdrew cash for lunch".into(),
        lines: vec![jx1_line1, jx1_line2],
    };
    let jx_same_ledger = jx1.clone();

    // Act
    state
        .engine
        .create_journal_transaction(&jx1)
        .await
        .expect("failed to create 1st journal transaction");
    let same_ledger_tx = state
        .engine
        .create_journal_transaction(&jx_same_ledger)
        .await;

    // Assert
    assert!(same_ledger_tx.is_err());
    let err_str = format!(
        "db error: ERROR: duplicate key value violates unique constraint \"journal_transaction_record_pkey\"\nDETAIL: Key (journal_id, \"timestamp\")=({}, {}) already exists.",
        state.journal.id,
        now
    );
    assert_eq!(
        same_ledger_tx.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Resource(OrmError::Internal(err_str.into())))
            .err()
            .unwrap()
    );
    let jxacts = state
        .engine
        .get_journal_transactions(None)
        .await
        .expect("unexpected search error");
    assert_eq!(
        jxacts.len(),
        1,
        "There is 1 journal transaction in the entire db"
    );
}

#[tokio::test]
async fn journal_transaction_creation_invalid() {
    // Arrange
    let fake_account_id = AccountId::new();
    let state = TestState::new().await;
    let bank = state
        .create_account(
            "1002",
            "Bank",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .unwrap();
    let jxact = state.simple_xact_model();
    let invalid_cases = [
        (
            {
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jxact.line1, jxact.line2];
                jx1
            },
            ServiceError::Validation(format!(
                "both ledger and account fields empty: transaction: JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}", 
                jxact.jx.journal_id, jxact.timestamp
            )),
            "line1: ledger_id and account_id are None" 
        ),
        (
            {
                let mut jx_line2 = jxact.line2.clone();
                jx_line2.ledger_id = Some(bank.id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jxact.line1, jx_line2];
                jx1
            },
            ServiceError::Validation(format!(
                "both ledger and account fields empty: transaction: JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}",
                jxact.jx.journal_id, jxact.timestamp
            )),
            "line2: ledger_id and account_id are None" 
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.ledger_id = Some(fake_account_id);
                jx_line1.account_id = Some(fake_account_id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jxact.line2];
                jx1
            },
            ServiceError::Validation(format!(
                "both ledger and account fields NOT empty: transaction: JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}",
                jxact.jx.journal_id, jxact.timestamp
            )),
            "line1: ledger_id and account_id are BOTH Some()" 
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.ledger_id = Some(bank.id);
                let mut jx_line2 = jxact.line2.clone();
                jx_line2.ledger_id = Some(fake_account_id);
                jx_line2.account_id = Some(fake_account_id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jx_line2];
                jx1
            },
            ServiceError::Validation(format!(
                "both ledger and account fields NOT empty: transaction: JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}",
                jxact.jx.journal_id, jxact.timestamp
            )),
            "line2: ledger_id and account_id are BOTH Some()" 
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.ledger_id = Some(fake_account_id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jxact.line2];
                jx1
            },
            ServiceError::EmptyRecord(format!(
                "account id: {fake_account_id}",
            )),
            "line1: ledger_id is fake",
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.ledger_id = Some(bank.id);
                let mut jx_line2 = jxact.line2.clone();
                jx_line2.ledger_id = Some(fake_account_id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jx_line2];
                jx1
            },
            ServiceError::EmptyRecord(format!(
                "account id: {fake_account_id}",
            )),
            "line2: ledger_id is fake",
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.account_id = Some(fake_account_id);
                let mut jx_line2 = jxact.line2.clone();
                jx_line2.ledger_id = Some(bank.id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jx_line2];
                jx1
            },
            ServiceError::EmptyRecord(format!(
                "account id: {fake_account_id}",
            )),
            "line1: account_id is fake",
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.ledger_id = Some(bank.id);
                let mut jx_line2 = jxact.line2.clone();
                jx_line2.account_id = Some(fake_account_id);
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1, jx_line2];
                jx1
            },
            ServiceError::EmptyRecord(format!(
                "account id: {fake_account_id}",
            )),
            "line2: account_id is fake",
        ),
    ];

    for (case, expected_error, msg) in invalid_cases {
        journal_transaction_invalid_common(&state, &case, expected_error, msg).await
    }
}

async fn journal_transaction_invalid_common(
    state: &TestState,
    jx1: &journal::transaction::Model,
    expected_error: ServiceError,
    msg: &str,
) {
    // Act
    let jx1_db = state.engine.create_journal_transaction(jx1).await;

    // Assert
    assert!(
        jx1_db.is_err(),
        "{msg}: journal transaction creation must fail if no valid account"
    );
    assert_eq!(jx1_db.err().unwrap(), expected_error, "{msg}");
    let jxacts = state
        .engine
        .get_journal_transactions(None)
        .await
        .expect(format!("{msg}: unexpected search error").as_str());
    assert_eq!(
        jxacts.len(),
        0,
        "{msg}: There are ZERO jrn xact(s) in the entire db"
    );
}

#[tokio::test]
async fn post_journal_transaction_happy_path() {
    // Arrange
    let state = TestState::new().await;
    let cash = state
        .create_account(
            "1001",
            "Cash",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("failed to creae Cash account");
    let bank = state
        .create_account(
            "1002",
            "Bank",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("failed to create Bank account");
    let jxact = state
        .create_journal_xact(Decimal::from(100), cash.id, bank.id, "Withdrew cash", None)
        .await
        .expect("failed to create journal transaction: Withdrew cash");

    // Act
    let posted = state
        .engine
        .post_transaction(jxact.id())
        .await
        .expect("failed to post journal transaction");

    // Assert
    assert!(posted, "the call to 'post' the journal tx succeeded");
    let bank_entries = state
        .engine
        .journal_entries(bank.id)
        .await
        .expect("failed to get journal enries for Bank ledger");
    let cash_entries = state
        .engine
        .journal_entries(cash.id)
        .await
        .expect("failed to get journal entries for Cash ledger");
    let jxact = state
        .engine
        .get_journal_transactions(Some(&vec![jxact.id()]))
        .await
        .expect("unexpected search error")
        .pop()
        .expect("unable to fetch journal transaction");
    let entry1 = state
        .engine
        .journal_entry_by_posting_ref(jxact.lines[0].posting_ref.unwrap())
        .await
        .expect("failed to fech journal entry #1 by posting ref.")
        .expect("journal entry #1 is empty");
    let entry2 = state
        .engine
        .journal_entry_by_posting_ref(jxact.lines[1].posting_ref.unwrap())
        .await
        .expect("failed to fech journal entry #2 by posting ref.")
        .expect("journal entry #2 is empty");
    for line in &jxact.lines {
        assert_eq!(
            line.state,
            TransactionState::Posted,
            "journal transaction line IS Posted"
        );
    }
    assert_eq!(
        cash_entries.len(),
        1,
        "there is ONE journal entry in the cash account"
    );
    assert_eq!(
        bank_entries.len(),
        1,
        "there is ONE journal entry in the bank account"
    );
    assert_eq!(
        entry1, cash_entries[0],
        "the 1st posting ref points to the DR account"
    );
    assert_eq!(
        entry2, bank_entries[0],
        "the 2nd posting ref points to the CR account"
    );
    let cr_account: ledger::ActiveModel = state
        .engine
        .get_ledgers(Some(&vec![bank.id]))
        .await
        .expect("unexpected search error")[0];
    let dr_account: ledger::ActiveModel = state
        .engine
        .get_ledgers(Some(&vec![cash.id]))
        .await
        .expect("unexpected search error")[0];
    assert_eq!(
        cr_account.id,
        jxact.lines[1].ledger_id.unwrap(),
        "ledger CR ac. matches journal"
    );
    assert_eq!(
        dr_account.id,
        jxact.lines[0].ledger_id.unwrap(),
        "ledger DR ac. matches journal"
    );
    assert_eq!(
        entry1.timestamp, jxact.timestamp,
        "ledger #1 datetime matches journal"
    );
    assert_eq!(
        entry2.timestamp, jxact.timestamp,
        "ledger #2 datetime matches journal"
    );
    assert_ne!(
        entry1.ledger_id.to_string(),
        entry2.ledger_id.to_string(),
        "accounts ARE different"
    );
    assert_ne!(
        jxact.lines[0].ledger_id.unwrap(),
        jxact.lines[1].ledger_id.unwrap(),
        "journal transaction dr and cr accounts are different"
    );
}

#[tokio::test]
async fn post_journal_transaction_unbalanced() {
    // Arrange
    let state = TestState::new().await;
    let bank = state
        .create_account(
            "1002",
            "Bank",
            LedgerType::Leaf,
            Some(state.general_ledger.root),
        )
        .await
        .expect("failed to create Bank account");
    let cr_line = journal::transaction::line::Model {
        journal_id: state.journal.id,
        timestamp: timestamp(),
        ledger_id: Some(bank.id),
        xact_type: XactType::Cr,
        amount: Decimal::ONE,
        ..Default::default()
    };
    let model = journal::transaction::Model {
        journal_id: state.journal.id,
        timestamp: timestamp(),
        explanation: "".into(),
        lines: vec![cr_line],
    };
    let jxact = state
        .engine
        .create_journal_transaction(&model)
        .await
        .expect("failed to create unbalanaced transaction");

    // Act
    let posted = state.engine.post_transaction(jxact.id()).await;

    // Assert
    assert!(posted.is_err(), "posting an unbalanced journal tx fails");
    assert_eq!(
        posted.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::Validation(
            "the Dr and Cr sides of the transaction must be equal".into(),
        ))
        .err()
        .unwrap()
    );
}

#[tokio::test]
async fn ledger_and_external_account() {
    // Arrange
    let state = TestState::new().await;
    let (_, ledger, account) = state.create_subsidiary("A/R").await;
    let mut jxact = state.simple_xact_model();
    jxact.line1.account_id = Some(account.id);
    jxact.line1.xact_type_external = Some("DF".into());
    jxact.line2.ledger_id = Some(ledger.id);

    // Act
    let jxact = state.create_simple_jxact(&jxact, "Widget sales").await;
    let posted = state
        .engine
        .post_transaction(jxact.id())
        .await
        .expect("failed to post transaction");

    // Assert
    assert!(posted, "posting of transaction succeeds");
    let cr_e = state
        .engine
        .journal_entries(ledger.id)
        .await
        .expect("failed to get journal enries for revenue ledger");
    let dr_e = state
        .engine
        .journal_entries(account.id)
        .await
        .expect("failed to get journal entries for subsidiary account");
    println!("dr_e: {:#?}\n", dr_e);
    println!("cr_e: {:#?}\n", cr_e);
    println!("memory_store: {:#?}\n", LedgerService::store(&state.engine));
    assert_eq!(
        cr_e.len(),
        1,
        "there is ONE journal entry in the ledger account"
    );
    assert_eq!(cr_e[0].amount, Decimal::ONE, "the CR amount equals 1.00");
    assert_eq!(
        cr_e[0].journal_ref,
        JournalTransactionId::new(jxact.journal_id, jxact.timestamp),
        "the CR journal reference points back to the transaction"
    );
    assert_eq!(
        dr_e.len(),
        1,
        "there is ONE journal entry in the external account"
    );
    assert_eq!(dr_e[0].amount, Decimal::ONE, "the DR amount equals 1.00");
    assert_eq!(
        dr_e[0].journal_ref,
        JournalTransactionId::new(jxact.journal_id, jxact.timestamp),
        "the DR journal reference points back to the transaction"
    );
}

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
        parent_id: Option<AccountId>,
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

    pub async fn create_subsidiary(
        &self,
        name: &'static str,
    ) -> (
        subsidiary_ledger::ActiveModel,
        ledger::ActiveModel,
        external::account::ActiveModel,
    ) {
        let ledg = self.create_ledger(name, LedgerType::Derived).await;
        let model = subsidiary_ledger::Model {
            name: name.into(),
            ledger_account_id: ledg.id,
        };
        let sub = self
            .engine
            .create_subsidiary_ledger(&model)
            .await
            .expect(format!("failed to create subsidiary ledger: {name}").as_str());

        let model = external::account::Model {
            subsidiary_ledger_id: sub.id,
            entity_type_code: "PE".into(),
            account_no: random_account_no().into(),
            date_opened: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        };
        let acc = self
            .engine
            .create_account(&model)
            .await
            .expect("failed to create external account");

        (sub, ledg, acc)
    }

    pub async fn create_journal(
        &self,
        code: &'static str,
        name: &'static str,
    ) -> Result<journal::ActiveModel, ServiceError> {
        let model = journal::Model {
            name: name.into(),
            code: code.into(),
        };

        self.engine.create_journal(&model).await
    }

    pub async fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: AccountId,
        account_cr_id: AccountId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal::transaction::ActiveModel, ServiceError> {
        let journal_id: JournalId = journal_id.unwrap_or(self.journal.id);
        let dr_line = journal::transaction::line::Model {
            journal_id: journal_id,
            timestamp: timestamp(),
            ledger_id: Some(account_dr_id),
            xact_type: XactType::Dr,
            amount: amount,
            ..Default::default()
        };
        let cr_line = journal::transaction::line::Model {
            journal_id: journal_id,
            timestamp: timestamp(),
            ledger_id: Some(account_cr_id),
            xact_type: XactType::Cr,
            amount: amount,
            ..Default::default()
        };
        let model = journal::transaction::Model {
            journal_id,
            timestamp: timestamp(),
            explanation: desc.into(),
            lines: vec![dr_line, cr_line],
        };

        self.engine.create_journal_transaction(&model).await
    }

    pub async fn create_simple_jxact(
        &self,
        tx: &SimpleJournalTransaction,
        desc: &str,
    ) -> journal::transaction::ActiveModel {
        let model = journal::transaction::Model {
            journal_id: tx.line1.journal_id,
            timestamp: timestamp(),
            explanation: desc.into(),
            lines: vec![tx.line1, tx.line2],
        };

        self.engine
            .create_journal_transaction(&model)
            .await
            .expect(format!("failed to create simple journal transaction: {desc}").as_str())
    }

    pub fn simple_xact_model(&self) -> SimpleJournalTransaction {
        let timestamp = timestamp();
        let line1 = journal::transaction::line::Model {
            journal_id: self.journal.id,
            timestamp,
            xact_type: XactType::Dr,
            amount: Decimal::ONE,
            ..Default::default()
        };
        let line2 = journal::transaction::line::Model {
            journal_id: self.journal.id,
            timestamp,
            xact_type: XactType::Cr,
            amount: Decimal::ONE,
            ..Default::default()
        };
        let jx = journal::transaction::Model {
            journal_id: self.journal.id,
            timestamp,
            explanation: "Withdrew cash for lunch".into(),
            lines: Vec::<journal::transaction::line::Model>::new(),
        };

        SimpleJournalTransaction {
            jx,
            line1,
            line2,
            timestamp,
        }
    }
}

pub struct SimpleJournalTransaction {
    jx: journal::transaction::Model,
    line1: journal::transaction::line::Model,
    line2: journal::transaction::line::Model,
    timestamp: NaiveDateTime,
}
