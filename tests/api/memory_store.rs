use std::str::FromStr;

use account_engine::{
    domain::{
        ids::JournalId, AccountId, ArrayCodeString, ArrayLongString, ArrayShortString,
        GeneralLedgerId, XactType,
    },
    entity::{
        accounting_period, general_ledger, interim_accounting_period, journal,
        jrnl::transaction::{journal_transaction, journal_transaction_line},
        ledger, InterimType, LedgerType, TransactionState,
    },
    memory_store::MemoryStore,
    orm::{error::OrmError, AccountRepository},
};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::timestamp;

#[tokio::test]
async fn non_existant_ledger() {
    // Arrange
    let state = TestState::new().await;
    let gl_id = GeneralLedgerId::new();

    // Act
    let res = state.store.search(Some(vec![gl_id])).await.unwrap();

    // Assert
    let all_gl: Vec<general_ledger::ActiveModel> = state.store.search(None).await.unwrap();
    assert_eq!(all_gl.len(), 1, "Only one GL in the list");
    assert_eq!(
        res.len(),
        0,
        "search for non-existent ledger returns nothing"
    );
}

#[tokio::test]
async fn duplicate_ledger_name() {
    // Arrange
    let state = TestState::new().await;
    let ledger1 = state.ledger;
    assert_eq!(
        ledger1.name.as_str(),
        "My Company",
        "Initial GL name is 'My Company'"
    );

    // Act
    let ledger2 = general_ledger::Model {
        name: ArrayLongString::from_str("My Company").unwrap(),
        currency_code: ArrayCodeString::from_str("USD").unwrap(),
    };
    let ledger2 = state.store.create(&ledger2).await;

    // Assert
    assert!(ledger2.is_ok(), "second identical GL created successfully");
    let ledger2 = ledger2.unwrap();
    assert_eq!(ledger2.name, ledger1.name, "Both GLs have identical names");
    assert_ne!(ledger1.id, ledger2.id, "Ledger IDs are different");
    let ledgers: Vec<general_ledger::ActiveModel> = state.store.search(None).await.unwrap();
    assert_eq!(ledgers.len(), 2, "There are 2 GLs in the list")
}

#[tokio::test]
async fn unique_account_number() {
    // Arrange
    let state = TestState::new().await;

    // Act
    let _ = state
        .create_account("1000", "Assets", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .expect("1st ledger creation failed");
    let assets_same_gl = state
        .create_account("1000", "Assets", LedgerType::Leaf, Some(state.ledger.root))
        .await;

    // Assert
    assert!(assets_same_gl.is_err(), "failed to create second account");
    assert_eq!(
        assets_same_gl.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord("account 1000".into(),))
            .err()
            .unwrap()
    );
    let gl1_accounts: Vec<ledger::ActiveModel> = state.store.search(None).await.unwrap();
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
            Some(state.ledger.root),
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
        Err::<(), OrmError>(OrmError::Constraint("ledger must have parent".into(),))
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
        .create_account("1000", "Assets", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .unwrap();
    let cash = state
        .create_account("1001", "Cash", LedgerType::Leaf, Some(assets.id))
        .await;

    // Assert
    assert!(cash.is_err(), "failed to create cash account");
    assert_eq!(
        cash.err().unwrap(),
        Err::<(), OrmError>(OrmError::Validation(
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
        .create_account("1000", "Assets", LedgerType::Leaf, Some(state.ledger.root))
        .await;

    // Act
    let assets_same_gl = state
        .create_account("1001", "Assets", LedgerType::Leaf, Some(state.ledger.root))
        .await;

    // Assert
    assert!(
        assets_original.is_ok(),
        "first account created successfully"
    );
    assert!(
        assets_same_gl.is_ok(),
        "second account with same name created successfully"
    );
    assert_eq!(
        assets_original.unwrap().name,
        assets_same_gl.unwrap().name,
        "account with duplicate name created successfully"
    );
    let gl1_accounts: Vec<ledger::ActiveModel> = state
        .store
        .search(None)
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
        .store
        .create(&period)
        .await
        .expect("1st fiscal year creation failed");
    let fy_duplicate = state.store.create(&period).await;

    // Assert
    assert!(
        fy_duplicate.is_err(),
        "duplicate fiscal year creation must fail"
    );
    assert_eq!(
        fy_duplicate.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord(
            "duplicate accounting period".into()
        ))
        .err()
        .unwrap()
    );
    let periods: Vec<accounting_period::ActiveModel> = state
        .store
        .search(None)
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
    let fy = state.store.create(&fy).await.unwrap();
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

    // Act
    let subperiods: Vec<interim_accounting_period::ActiveModel> = state
        .store
        .search(None)
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
        .store
        .create(&j1)
        .await
        .expect("failed to create first jounral");
    let journal2 = state.store.create(&j2).await;

    // Assert
    assert!(journal2.is_err(), "failed to create second ledger");
    assert_eq!(
        journal2.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord(
            "duplicate Journal Id or Code".into()
        ))
        .err()
        .unwrap()
    );
    let journals: Vec<journal::ActiveModel> = state
        .store
        .search(None)
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
        .create_account("1001", "Cash", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .unwrap();
    let bank1 = state
        .create_account("1002", "Bank", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .unwrap();

    let now = timestamp();
    let jx1_line1 = journal_transaction_line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        ledger_id: Some(cash1.id),
        account_id: None,
        xact_type: XactType::Dr,
        state: TransactionState::Pending,
        amount: Decimal::ZERO,
        posting_ref: None,
    };
    let jx1_line2 = journal_transaction_line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        ledger_id: Some(bank1.id),
        account_id: None,
        xact_type: XactType::Cr,
        state: TransactionState::Pending,
        amount: Decimal::ZERO,
        posting_ref: None,
    };
    let jx1 = journal_transaction::Model {
        journal_id: state.journal.id,
        timestamp: now,
        explanation: "Withdrew cash for lunch".into(),
        lines: vec![jx1_line1, jx1_line2],
    };
    let jx_same_ledger = jx1.clone();

    // Act
    state
        .store
        .create(&jx1)
        .await
        .expect("failed to create 1st journal transaction");
    let same_ledger_tx = state.store.create(&jx_same_ledger).await;

    // Assert
    assert!(same_ledger_tx.is_err());
    let err_str = format!(
        "journal transaction exists: JournalTransactionId {{ Journal ID: {}, Timestamp: {} }}",
        state.journal.id, now
    );
    assert_eq!(
        same_ledger_tx.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord(err_str.into()))
            .err()
            .unwrap()
    );
    let jxacts: Vec<journal_transaction::ActiveModel> = state
        .store
        .search(None)
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
        .create_account("1002", "Bank", LedgerType::Leaf, Some(state.ledger.root))
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
            OrmError::Internal(format!(
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
            OrmError::Internal(format!(
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
            OrmError::Internal(format!(
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
            OrmError::Internal(format!(
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
            OrmError::RecordNotFound(format!(
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
            OrmError::RecordNotFound(format!(
                "account id: {fake_account_id}",
            )),
            "line2: ledger_id is fake",
        ),
        // (
        //     {
        //         let mut jx_line1 = jx_line1.clone();
        //         jx_line1.account_id = Some(fake_account_id);
        //         let mut jx_line2 = jx_line2.clone();
        //         jx_line2.ledger_id = Some(bank.id);
        //         let mut jx1 = jx1.clone();
        //         jx1.lines = vec![jx_line1, jx_line2]; 
        //         jx1
        //     },
        //     OrmError::RecordNotFound(format!(
        //         "account id: {fake_account_id}",
        //     )),
        //     "line1: account_id is fake",
        // ),
        // (
        //     {
        //         let mut jx_line1 = jx_line1.clone();
        //         jx_line1.ledger_id = Some(bank.id);
        //         let mut jx_line2 = jx_line2.clone();
        //         jx_line2.account_id = Some(fake_account_id);
        //         let mut jx1 = jx1.clone();
        //         jx1.lines = vec![jx_line1, jx_line2]; 
        //         jx1
        //     },
        //     OrmError::RecordNotFound(format!(
        //         "account id: {fake_account_id}",
        //     )),
        //     "line2: account_id is fake",
        // ),
    ];

    for (case, expected_error, msg) in invalid_cases {
        journal_transaction_invalid_common(&state, &case, expected_error, msg).await
    }
}

async fn journal_transaction_invalid_common(
    state: &TestState,
    jx1: &journal_transaction::Model,
    expected_error: OrmError,
    msg: &str,
) {
    // Act
    let jx1_db = state.store.create(jx1).await;

    // Assert
    assert!(
        jx1_db.is_err(),
        "{msg}: journal transaction creation must fail if no valid account"
    );
    assert_eq!(jx1_db.err().unwrap(), expected_error, "{msg}");
    let jxacts: Vec<journal_transaction::ActiveModel> = state
        .store
        .search(None)
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
        .create_account("1001", "Cash", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .expect("failed to creae Cash account");
    let bank = state
        .create_account("1002", "Bank", LedgerType::Leaf, Some(state.ledger.root))
        .await
        .expect("failed to create Bank account");
    let jxact = state
        .create_journal_xact(Decimal::from(100), cash.id, bank.id, "Withdrew cash", None)
        .await
        .expect("failed to create journal transaction: Withdrew cash");

    // Act
    let posted = state.store.post_journal_transaction(jxact.id()).await;

    // Assert
    assert!(posted, "the call to 'post' the journal tx succeeded");
    let bank_entries = state.store.journal_entries_by_account_id(bank.id).await;
    let cash_entries = state.store.journal_entries_by_account_id(cash.id).await;
    let jxact = &state
        .store
        .search(Some(vec![jxact.id()]))
        .await
        .expect("unexpected search error")[0];
    let entry1 = state
        .store
        .journal_entry_by_ref(jxact.lines[0].posting_ref.unwrap())
        .await
        .expect("failed to retrieve journal entry #1 using the posting ref.");
    let entry2 = state
        .store
        .journal_entry_by_ref(jxact.lines[1].posting_ref.unwrap())
        .await
        .expect("failed to retrieve journal entry #2 using the posting ref.");
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
    let cr_account = state
        .store
        .search(Some(vec![bank.id]))
        .await
        .expect("unexpected search error")[0];
    let dr_account = state
        .store
        .search(Some(vec![cash.id]))
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
        "ournal transaction dr and cr account are different"
    );
}

pub struct TestState {
    pub store: MemoryStore,
    pub ledger: general_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
}

impl TestState {
    pub async fn new() -> TestState {
        let store = MemoryStore::new();
        let ledger = general_ledger::Model {
            name: ArrayLongString::from_str("My Company").unwrap(),
            currency_code: ArrayCodeString::from_str("USD").unwrap(),
        };
        let ledger = store.create(&ledger).await.unwrap();
        let journal = journal::Model {
            name: "General Journal".into(),
            code: "G".into(),
        };
        let journal = store.create(&journal).await.unwrap();

        Self {
            store,
            ledger,
            journal,
        }
    }

    pub async fn create_account(
        &self,
        number: &'static str,
        name: &'static str,
        typ: LedgerType,
        parent_id: Option<AccountId>,
    ) -> Result<ledger::ActiveModel, OrmError> {
        let account = ledger::Model {
            ledger_no: ArrayShortString::from_str(number).unwrap(),
            ledger_type: typ,
            parent_id,
            name: ArrayLongString::from_str(name).unwrap(),
            currency_code: None,
        };

        self.store.create(&account).await
    }

    pub async fn create_journal(
        &self,
        code: &'static str,
        name: &'static str,
    ) -> Result<journal::ActiveModel, OrmError> {
        let model = journal::Model {
            name: name.into(),
            code: code.into(),
        };

        self.store.create(&model).await
    }

    pub async fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: AccountId,
        account_cr_id: AccountId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        let journal_id: JournalId = journal_id.unwrap_or(self.journal.id);
        let dr_line = journal_transaction_line::Model {
            journal_id: journal_id,
            timestamp: timestamp(),
            ledger_id: Some(account_dr_id),
            account_id: None,
            xact_type: XactType::Dr,
            state: TransactionState::Pending,
            amount: amount,
            posting_ref: None,
        };
        let cr_line = journal_transaction_line::Model {
            journal_id: journal_id,
            timestamp: timestamp(),
            ledger_id: Some(account_cr_id),
            account_id: None,
            xact_type: XactType::Cr,
            state: TransactionState::Pending,
            amount: amount,
            posting_ref: None,
        };
        let model = journal_transaction::Model {
            journal_id,
            timestamp: timestamp(),
            explanation: desc.into(),
            lines: vec![dr_line, cr_line],
        };

        self.store.create(&model).await
    }

    pub fn simple_xact_model(&self) -> SimpleJournalTransaction {
        let timestamp = timestamp();
        let line1 = journal_transaction_line::Model {
            journal_id: self.journal.id,
            timestamp,
            ledger_id: None,
            account_id: None,
            xact_type: XactType::Dr,
            state: TransactionState::Pending,
            amount: Decimal::ZERO,
            posting_ref: None,
        };
        let line2 = journal_transaction_line::Model {
            journal_id: self.journal.id,
            timestamp,
            ledger_id: None,
            account_id: None,
            xact_type: XactType::Cr,
            state: TransactionState::Pending,
            amount: Decimal::ZERO,
            posting_ref: None,
        };
        let jx = journal_transaction::Model {
            journal_id: self.journal.id,
            timestamp,
            explanation: "Withdrew cash for lunch".into(),
            lines: Vec::<journal_transaction_line::Model>::new(),
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
    jx: journal_transaction::Model,
    line1: journal_transaction_line::Model,
    line2: journal_transaction_line::Model,
    timestamp: NaiveDateTime,
}
