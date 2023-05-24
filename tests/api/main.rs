use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use account_engine::{
    accounting::{
        period::{InterimPeriod, InterimType},
        Account, Journal, JournalTransaction, JournalTransactionModel, Ledger, LedgerType,
    },
    memory_store::MemoryStore,
    storage::{AccountEngineStorage, StorageError},
};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use rusty_money::iso;

#[test]
fn test_non_existant_ledger() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let _ = db.new_ledger("My Company", iso::USD);
    let res = db.ledgers_by_name("Other Company");

    // Assert
    assert_eq!(db.ledgers().len(), 1, "Only one ledger in the list");
    assert_eq!(
        res.len(),
        0,
        "search for non-existent ledger returns nothing"
    );
}

#[test]
fn test_unique_ledger_name() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let ledger1 = db.new_ledger("My Company", iso::USD);
    let ledger2 = db.new_ledger("My Company", iso::EUR);

    // Assert
    assert!(ledger1.is_ok(), "first ledger created successfully");
    assert!(ledger2.is_err(), "failed to create second ledger");
    assert_eq!(
        ledger2.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate ledger name".into()
        ))
        .err()
        .unwrap()
    );
    assert_eq!(db.ledgers().len(), 1, "Only one ledger in the list")
}

#[test]
fn test_add_subsidiary_ledger() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let subsidiary = db.new_ledger("Sales Ledger", iso::EUR).unwrap();
    let ledger = db.new_ledger("My Company", iso::USD).unwrap();
    let _ = db.add_subsidiary(&ledger, *subsidiary);

    // Assert
    let ledgers = db.ledgers_by_name(&ledger.name);
    assert_eq!(db.ledgers().len(), 2, "There are two ledgers in the list");
    assert_eq!(
        ledgers[0].subsidiaries.len(),
        1,
        "There is one subsidiary ledger in the main ledger"
    );
    assert_eq!(
        ledgers[0].subsidiaries[0].name, "Sales Ledger",
        "The Sales Ledger is a subsidiary ledger of 'My Company'"
    )
}

#[test]
fn test_unique_account_number() {
    // Arrance
    let db = MemoryStore::new();
    let ledger = db.new_ledger("My Company", iso::USD).unwrap();
    let ledger2 = db.new_ledger("Another Company", iso::USD).unwrap();

    // Act
    let assets = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let liabilities = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let assets2 = db.new_account(
        &ledger2,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );

    // Assert
    assert!(assets.is_ok(), "first account created successfully");
    assert!(liabilities.is_err(), "failed to create second account");
    assert_eq!(
        liabilities.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate account number".into()
        ))
        .err()
        .unwrap()
    );
    assert_eq!(
        db.accounts(&ledger).len(),
        1,
        "Only one account in the ledger"
    );
    assert!(
        assets2.is_ok(),
        "account with duplicate numbe, but in DIFFERENT ledger created successfully"
    );
}

#[test]
fn test_duplicate_account_name() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let ledger = db.new_ledger("My Company", iso::USD).unwrap();
    let assets = db.new_account(
        &ledger,
        "Assets",
        "1000",
        LedgerType::Intermediate,
        Some(iso::USD),
    );
    let assets2 = db.new_account(
        &ledger,
        "Assets",
        "1100",
        LedgerType::Intermediate,
        Some(iso::USD),
    );

    // Assert
    assert!(assets.is_ok(), "first account created successfully");
    assert!(assets2.is_ok(), "second account created successfully");
    assert_eq!(
        assets.unwrap().name,
        assets2.unwrap().name,
        "account with duplicate name created successfully"
    );
    assert_eq!(
        db.accounts(&ledger).len(),
        2,
        "Both accounts appear in the ledger"
    )
}

#[test]
fn test_unique_accounting_period() {
    // Arrance
    let db = MemoryStore::new();

    // Act
    let fy = db.new_period(
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        InterimType::CalendarMonth,
    );
    let fy_duplicate = db.new_period(
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        InterimType::CalendarMonth,
    );

    // Assert
    assert!(fy.is_ok(), "first fiscal year created successfully");
    assert!(
        fy_duplicate.is_err(),
        "duplicate fiscal year creation failed"
    );
    assert_eq!(
        fy_duplicate.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate accounting period".into()
        ))
        .err()
        .unwrap()
    );
    assert_eq!(
        db.periods().unwrap().len(),
        1,
        "Only one period in the list"
    )
}

#[test]
fn test_create_accounting_period_calendar() {
    // Arrance
    let db = MemoryStore::new();
    let periods = vec![
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 1, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 2, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 2, 28).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 3, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 3, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 4, 30).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 5, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 6, 30).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 7, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 8, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 9, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 9, 30).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 10, 31).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 11, 30).unwrap(),
        },
        InterimPeriod {
            start: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        },
    ];

    // Act
    let fy2023 = db
        .new_period(
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            InterimType::CalendarMonth,
        )
        .unwrap();

    // Assert
    assert_eq!(
        fy2023.periods.len(),
        12,
        "12 periods in Calendar Month period"
    );
    assert_eq!(
        fy2023.period_start,
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        "period start is Jan 1, 2023"
    );
    assert_eq!(
        fy2023.period_end,
        NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
        "period end is Dec 31, 2023"
    );
    assert_eq!(fy2023.periods, periods, "periods calculated correctly")
}

#[test]
fn test_unique_journal_name() {
    // Arrance
    let db = MemoryStore::new();
    let ledger1 = db.new_ledger("My Company", iso::USD).unwrap();
    let j1 = Journal {
        name: "General".into(),
        code: "G".into(),
        ledger: ledger1.clone().into(),
        xacts: Vec::<JournalTransaction>::new(),
    };
    let j2 = Journal {
        name: "Sales".into(),
        code: "G".into(),
        ledger: ledger1.into(),
        xacts: Vec::<JournalTransaction>::new(),
    };
    let ledger2 = db.new_ledger("Other Company", iso::USD).unwrap();
    let mut j3 = j2.clone();
    j3.ledger = ledger2.into();

    // Act
    let journal1 = db.new_journal(&j1);
    let journal2 = db.new_journal(&j2);
    let journal3 = db.new_journal(&j3);

    // Assert
    assert!(journal1.is_ok(), "first journal created successfully");
    assert!(journal2.is_err(), "failed to create second ledger");
    assert_eq!(
        journal2.err().unwrap(),
        Err::<(), StorageError>(StorageError::DuplicateRecord(
            "duplicate journal code".into()
        ))
        .err()
        .unwrap()
    );
    assert!(
        journal3.is_ok(),
        "jrnl with same code in ANOTHER ledger created successfully"
    );
    assert_eq!(db.journals().len(), 2, "Two journals created");
    assert_eq!(
        db.journals_by_ledger("My Company").len(),
        1,
        "One journal is in the first ledger"
    );
    assert_eq!(
        db.journals_by_ledger("Other Company").len(),
        1,
        "The other journal is in the second ledger"
    );
}

#[test]
fn test_journal_transaction_creation() {
    // Arrange
    let state = TestState::new();
    let ledger2 = state.db.new_ledger("Other Company", iso::USD).unwrap();
    let _cash = state.create_account("1001");
    let _bank = state.create_account("1002");
    let _cash = state
        .db
        .new_account(&ledger2, "Cash", "1001", LedgerType::Leaf, Some(iso::USD))
        .unwrap();
    let _bank = state
        .db
        .new_account(&ledger2, "Bank", "1002", LedgerType::Leaf, Some(iso::USD))
        .unwrap();
    let j1 = Journal {
        name: "General".into(),
        code: "G".into(),
        ledger: state.ledger.clone().into(),
        xacts: Vec::<JournalTransaction>::new(),
    };
    let mut j2 = j1.clone();
    j2.ledger = ledger2.clone().into();
    let journal2 = state.db.new_journal(&j2).unwrap();

    let now = timestamp();
    let jx1 = JournalTransactionModel {
        journal: Arc::new(state.journal.clone()),
        timestamp: now,
        posted: false,
        amount: Decimal::ZERO,
        acc_no_dr: "1001".into(),
        acc_no_cr: "1002".into(),
        description: "Withdrew cash for lunch".into(),
        posting_ref: None,
    };
    let jx_same_ledger = jx1.clone();
    let mut jx_other_ledger = jx1.clone();
    jx_other_ledger.journal = Arc::new(journal2.clone());

    // Act
    let jx1_db = state.db.new_journal_transaction(jx1);
    let jx_same_db = state.db.new_journal_transaction(jx_same_ledger);
    let jx_other_db = state.db.new_journal_transaction(jx_other_ledger);

    // Assert
    assert!(jx1_db.is_ok(), "jx was created successfully");
    assert!(
        jx_same_db.is_ok(),
        "jx (same id in same ledger) was successfull"
    );
    assert!(
        jx_other_db.is_ok(),
        "jx (same id in different ledger) was successfull"
    );
    let jx1_db = jx1_db.unwrap();
    let jx_same_db = jx_same_db.unwrap();
    let jx_other_db = jx_other_db.unwrap();
    assert_ne!(jx1_db.id, jx_same_db.id, "transaction ids are different");
    assert_ne!(jx1_db.id, jx_other_db.id, "transaction ids are different");
    assert_ne!(
        jx_same_db.id, jx_other_db.id,
        "transaction ids are different"
    );
    assert_eq!(
        state.db.journal_transactions().len(),
        3,
        "There are 3 jx(s) in the entire db"
    );
    assert_eq!(
        state
            .db
            .journal_transactions_by_ledger(&state.ledger.name)
            .len(),
        2,
        "There are two transaction when searching by 1st ledger"
    );
    assert_eq!(
        state.db.journal_transactions_by_ledger(&ledger2.name).len(),
        1,
        "There is only one transaction when searching by 2nd ledger"
    );
}

#[test]
fn test_journal_transaction_creation_no_valid_account() {
    // Arrange
    let state = TestState::new();
    let _bank = state
        .db
        .new_account(
            &state.ledger,
            "Bank",
            "1002",
            LedgerType::Leaf,
            Some(iso::USD),
        )
        .unwrap();
    let now = timestamp();
    let jx1 = JournalTransactionModel {
        journal: Arc::new(state.journal.clone()),
        timestamp: now,
        posted: false,
        amount: Decimal::ZERO,
        acc_no_dr: "1001".into(),
        acc_no_cr: "1002".into(),
        description: "Withdrew cash for lunch".into(),
        posting_ref: None,
    };

    // Act
    let jx1_db = state.db.new_journal_transaction(jx1);

    // Assert
    assert!(jx1_db.is_err(), "jx was created successfully");
    assert_eq!(
        state.db.journal_transactions().len(),
        0,
        "There are ZERO jx(s) in the entire db"
    );
}

struct TestState {
    db: MemoryStore,
    ledger: Box<Ledger>,
    journal: Journal,
}

impl TestState {
    fn new() -> TestState {
        let db = MemoryStore::new();
        let ledger = db.new_ledger("My Company", iso::USD).unwrap();
        let journal = Journal {
            name: "General Journal".into(),
            code: "G".into(),
            ledger: Arc::from(ledger.clone()),
            xacts: Vec::<JournalTransaction>::new(),
        };

        Self {
            db,
            ledger,
            journal,
        }
    }

    fn create_account(&self, number: &str) -> Account {
        self.db
            .new_account(
                &self.ledger,
                number,
                number,
                LedgerType::Leaf,
                Some(iso::USD),
            )
            .unwrap()
    }
}

fn timestamp() -> NaiveDateTime {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch");
    let naive =
        NaiveDateTime::from_timestamp_opt(now.as_secs() as i64, now.subsec_nanos()).unwrap();

    naive
}
