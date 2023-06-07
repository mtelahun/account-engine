use std::time::{SystemTime, UNIX_EPOCH};

use account_engine::{
    domain::{ids::JournalId, AccountId, LedgerId},
    entity::{
        accounting_period, general_ledger, interim_accounting_period, journal, journal_transaction,
        ledger, InterimType, LedgerType, TransactionState,
    },
    memory_store::MemoryStore,
    orm::{error::OrmError, RepositoryOrm},
};
use arrayvec::ArrayString;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use rusty_money::iso;

#[test]
fn test_non_existant_ledger() {
    // Arrange
    let state = TestState::new();
    let gl_id = LedgerId::new();

    // Act
    let res = state.db.search(Some(&[gl_id]));

    // Assert
    let all_gl: Vec<general_ledger::ActiveModel> = state.db.search(None);
    assert_eq!(all_gl.len(), 1, "Only one GL in the list");
    assert_eq!(
        res.len(),
        0,
        "search for non-existent ledger returns nothing"
    );
}

#[test]
fn test_duplicate_ledger_name() {
    // Arrange
    let state = TestState::new();
    let ledger1 = state.ledger;
    assert_eq!(
        ledger1.name.as_str(),
        "My Company",
        "Initial GL name is 'My Company'"
    );

    // Act
    let ledger2 = general_ledger::Model {
        name: ArrayString::<256>::from("My Company").unwrap(),
        currency: *iso::USD,
    };
    let ledger2 = state.db.create(&ledger2);

    // Assert
    assert!(ledger2.is_ok(), "second identical GL created successfully");
    let ledger2 = ledger2.unwrap();
    assert_eq!(ledger2.name, ledger1.name, "Both GLs have identical names");
    assert_ne!(ledger1.id, ledger2.id, "Ledger IDs are different");
    let ledgers: Vec<general_ledger::ActiveModel> = state.db.search(None);
    assert_eq!(ledgers.len(), 2, "There are 2 GLs in the list")
}

#[test]
fn test_unique_account_number() {
    // Arrange
    let state = TestState::new();
    let ledger2 = general_ledger::Model {
        name: ArrayString::<256>::from("Other Company").unwrap(),
        currency: *iso::USD,
    };
    let ledger2 = state.db.create(&ledger2).unwrap();

    // Act
    let assets_original = state.create_account("1000", "Assets", None);
    let assets_same_gl = state.create_account("1000", "Assets", None);
    let assets_different_gl = ledger::Model {
        general_ledger_id: ledger2.id,
        ledger_no: ArrayString::<64>::from("1000").unwrap(),
        ledger_type: LedgerType::Leaf,
        name: ArrayString::<256>::from("Assets").unwrap(),
        currency: None,
    };
    let assets_different_gl = state.db.create(&assets_different_gl);

    // Assert
    assert!(
        assets_original.is_ok(),
        "first account created successfully"
    );
    assert!(assets_same_gl.is_err(), "failed to create second account");
    assert_eq!(
        assets_same_gl.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord("account 1000".into(),))
            .err()
            .unwrap()
    );
    let gl1_accounts: Vec<ledger::ActiveModel> = state
        .db
        .search(None)
        .into_iter()
        .filter(|l: &ledger::ActiveModel| l.general_ledger_id == state.ledger.id)
        .collect();
    assert_eq!(
        gl1_accounts.len(),
        2,
        "Only one account (+ root) in the 1st ledger"
    );
    assert!(
        assets_different_gl.is_ok(),
        "account with duplicate number, but in DIFFERENT ledger created successfully"
    );
}

#[test]
fn test_duplicate_account_name_ok() {
    // Arrange
    let state = TestState::new();
    let assets_original = state.create_account("1000", "Assets", None);

    // Act
    let assets_same_gl = state.create_account("1001", "Assets", None);

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
    let gl1_accounts: Vec<ledger::ActiveModel> = state.db.search(None);
    assert_eq!(
        gl1_accounts.len(),
        3,
        "Both accounts (+ root) appear in the ledger"
    );
}

#[test]
fn test_unique_accounting_period() {
    // Arrance
    let state = TestState::new();
    let period = accounting_period::Model {
        ledger_id: state.ledger.id,
        fiscal_year: 2023,
        period_start: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        period_type: InterimType::CalendarMonth,
    };

    // Act
    let fy = state.db.create(&period);
    let fy_duplicate = state.db.create(&period);

    // Assert
    assert!(fy.is_ok(), "first fiscal year created successfully");
    assert!(
        fy_duplicate.is_err(),
        "duplicate fiscal year creation should failed"
    );
    assert_eq!(
        fy_duplicate.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord(
            "duplicate accounting period".into()
        ))
        .err()
        .unwrap()
    );
    let periods: Vec<accounting_period::ActiveModel> = state.db.search(None);
    assert_eq!(periods.len(), 1, "Only one period in the list")
}

#[test]
fn test_create_accounting_period_calendar() {
    // Arrange
    let state = TestState::new();
    let fy = accounting_period::Model {
        ledger_id: state.ledger.id,
        fiscal_year: 2023,
        period_start: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        period_type: InterimType::CalendarMonth,
    };
    let fy = state.db.create(&fy).unwrap();
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
    let subperiods: Vec<interim_accounting_period::ActiveModel> = state.db.search(None);

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

#[test]
fn test_unique_journal_name() {
    // Arrange
    let state = TestState::new();
    let gl1 = &state.ledger;
    let gl2 = general_ledger::Model {
        name: ArrayString::<256>::from("Other Company").unwrap(),
        currency: *iso::USD,
    };
    let gl2 = state.db.create(&gl2).unwrap();
    let j1 = journal::Model {
        name: "General".into(),
        code: "S".into(),
        ledger_id: gl1.id,
    };
    let j2 = journal::Model {
        name: "Sales".into(),
        code: "S".into(),
        ledger_id: gl1.id,
    };
    let mut j3 = j2.clone();
    j3.ledger_id = gl2.id;

    // Act
    let journal1 = state.db.create(&j1);
    let journal2 = state.db.create(&j2);
    let journal3 = state.db.create(&j3);

    // Assert
    assert!(journal1.is_ok(), "first journal created successfully");
    assert!(journal2.is_err(), "failed to create second ledger");
    assert_eq!(
        journal2.err().unwrap(),
        Err::<(), OrmError>(OrmError::DuplicateRecord(
            "duplicate Journal Id or Code".into()
        ))
        .err()
        .unwrap()
    );
    assert!(
        journal3.is_ok(),
        "jrnl with same code in ANOTHER ledger created successfully"
    );
    let journals: Vec<journal::ActiveModel> = state.db.search(None);
    assert_eq!(
        journals.len(),
        3,
        "Two (+1 by the test harness) journals created"
    );
    let jrn_gl1 = journals.clone();
    let jrn_gl1: Vec<journal::ActiveModel> = jrn_gl1
        .into_iter()
        .filter(|j| j.ledger_id == gl1.id)
        .collect();
    assert_eq!(
        jrn_gl1.len(),
        2,
        "One journal (+1 by the test harness) is in the first ledger"
    );
    let jrn_gl2 = journals.clone();
    let jrn_gl2: Vec<journal::ActiveModel> = jrn_gl2
        .into_iter()
        .filter(|j| j.ledger_id == gl2.id)
        .collect();
    assert_eq!(
        jrn_gl2.len(),
        1,
        "The other journal is in the second ledger"
    );
}

#[test]
fn test_journal_transaction_creation() {
    // Arrange
    let state = TestState::new();
    let gl2 = general_ledger::Model {
        name: ArrayString::<256>::from("Other Company").unwrap(),
        currency: *iso::USD,
    };
    let gl2 = state.db.create(&gl2).unwrap();
    let cash1 = state.create_account("1001", "Cash", None).unwrap();
    let bank1 = state.create_account("1002", "Bank", None).unwrap();
    let cash2 = state.create_account("1001", "Cash", Some(gl2.id)).unwrap();
    let bank2 = state.create_account("1002", "Bank", Some(gl2.id)).unwrap();
    let journal2 = state
        .create_journal("G", "General Journal", Some(gl2.id))
        .unwrap();

    let now = timestamp();
    let jx1 = journal_transaction::Model {
        journal_id: state.journal.id,
        timestamp: now,
        state: TransactionState::Pending,
        amount: Decimal::ZERO,
        description: "Withdrew cash for lunch".into(),
        posting_ref: None,
        account_dr_id: cash1.id,
        account_cr_id: bank1.id,
    };
    let jx_same_ledger = jx1.clone();
    let mut jx_other_ledger = jx1.clone();
    jx_other_ledger.journal_id = journal2.id;
    jx_other_ledger.account_cr_id = bank2.id;
    jx_other_ledger.account_dr_id = cash2.id;

    // Act
    let jx1 = state.db.create(&jx1);
    let jx_same_ledger = state.db.create(&jx_same_ledger);
    let jx_other_ledger = state.db.create(&jx_other_ledger);

    // Assert
    assert!(jx1.is_ok(), "jx was created successfully");
    assert!(
        jx_same_ledger.is_ok(),
        "jx (same id in same ledger) was successfull"
    );
    assert!(
        jx_other_ledger.is_ok(),
        "jx (same id in different ledger) was successfull"
    );
    let jx1 = jx1.unwrap();
    let jx_same_ledger = jx_same_ledger.unwrap();
    let jx_other_ledger = jx_other_ledger.unwrap();
    assert_ne!(jx1.id, jx_same_ledger.id, "transaction ids are different");
    assert_ne!(jx1.id, jx_other_ledger.id, "transaction ids are different");
    assert_ne!(
        jx_same_ledger.id, jx_other_ledger.id,
        "transaction ids are different"
    );
    let jxacts: Vec<journal_transaction::ActiveModel> = state.db.search(None);
    assert_eq!(jxacts.len(), 3, "There are 3 jx(s) in the entire db");
    let jxacts1: Vec<journal_transaction::ActiveModel> = jxacts
        .clone()
        .into_iter()
        .filter(|jx| jx.journal_id == state.journal.id)
        .collect();
    let jxacts2: Vec<journal_transaction::ActiveModel> = jxacts
        .into_iter()
        .filter(|jx| jx.journal_id == journal2.id)
        .collect();
    assert_eq!(
        jxacts1.len(),
        2,
        "There are two transaction when searching by 1st ledger"
    );
    assert_eq!(
        jxacts2.len(),
        1,
        "There is only one transaction when searching by 2nd ledger"
    );
}

#[test]
fn test_journal_transaction_creation_no_valid_account() {
    // Arrange
    let state = TestState::new();
    let account_dr_id = AccountId::new();
    let bank = state.create_account("1002", "Bank", None).unwrap();
    let now = timestamp();
    let jx1 = journal_transaction::Model {
        journal_id: state.journal.id,
        timestamp: now,
        state: TransactionState::Pending,
        amount: Decimal::ZERO,
        account_dr_id,
        account_cr_id: bank.id,
        description: "Withdrew cash for lunch".into(),
        posting_ref: None,
    };

    // Act
    let jx1_db = state.db.create(&jx1);

    // Assert
    assert!(jx1_db.is_err(), "jx was created successfully");
    assert_eq!(
        jx1_db.err().unwrap(),
        Err::<(), OrmError>(OrmError::RecordNotFound(format!(
            "account id: {}",
            account_dr_id
        )))
        .err()
        .unwrap()
    );
    let jxacts: Vec<journal_transaction::ActiveModel> = state.db.search(None);
    assert_eq!(
        jxacts.len(),
        0,
        "There are ZERO jrn xact(s) in the entire db"
    );
}

#[test]
fn test_post_journal_transaction_happy_path() {
    // Arrange
    let state = TestState::new();
    let cash = state.create_account("1001", "Cash", None).unwrap();
    let bank = state.create_account("1002", "Bank", None).unwrap();
    let jxact = state
        .create_journal_xact(Decimal::from(100), cash.id, bank.id, "Withdrew cash", None)
        .unwrap();

    // Act
    let posted = state.db.post_journal_transaction(jxact.id);

    // Assert
    assert!(posted, "the call to 'post' the journal tx succeeded");
    let bank_entries = state.db.journal_entries_by_account_id(bank.id);
    let cash_entries = state.db.journal_entries_by_account_id(cash.id);
    let jxact = &state.db.search(Some(&[jxact.id]))[0];
    let entries = state.db.journal_entries_by_ref(jxact.posting_ref.unwrap());
    assert_eq!(
        jxact.state,
        TransactionState::Posted,
        "journal tx IS Posted"
    );
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
        entries[0], bank_entries[0],
        "the 1st posting ref points to the CR account"
    );
    assert_eq!(
        entries[1], cash_entries[0],
        "the 2nd posting ref points to the DR account"
    );
    let cr_account = state.db.search(Some(&[bank.id]))[0];
    let dr_account = state.db.search(Some(&[cash.id]))[0];
    assert_eq!(
        cr_account.id, jxact.account_cr_id,
        "ledger CR ac. matches journal"
    );
    assert_eq!(
        dr_account.id, jxact.account_dr_id,
        "ledger DR ac. matches journal"
    );
    assert_eq!(
        entries[0].datetime, jxact.timestamp,
        "ledger datetime matches journal"
    );
    assert_ne!(
        entries[1].ledger_no.to_string(),
        entries[0].ledger_no.to_string(),
        "accounts ARE different"
    );
    assert_eq!(
        entries[1].datetime, entries[0].datetime,
        "both journal entries' timestamp ARE equal"
    );
    assert_ne!(
        jxact.account_dr_id, jxact.account_cr_id,
        "dr and cr account are different"
    );
}

struct TestState {
    db: MemoryStore,
    ledger: general_ledger::ActiveModel,
    journal: journal::ActiveModel,
}

impl TestState {
    fn new() -> TestState {
        let db = MemoryStore::new();
        let ledger = general_ledger::Model {
            name: ArrayString::<256>::from("My Company").unwrap(),
            currency: *iso::USD,
        };
        let ledger = db.create(&ledger).unwrap();
        let journal = journal::Model {
            name: "General Journal".into(),
            code: "G".into(),
            ledger_id: ledger.id,
        };
        let journal = db.create(&journal).unwrap();

        Self {
            db,
            ledger,
            journal,
        }
    }

    fn create_account(
        &self,
        number: &'static str,
        name: &'static str,
        ledger_id: Option<LedgerId>,
    ) -> Result<ledger::ActiveModel, OrmError> {
        let ledger_id = ledger_id.unwrap_or(self.ledger.id);
        let account = ledger::Model {
            general_ledger_id: ledger_id,
            ledger_no: ArrayString::<64>::from(number).unwrap(),
            ledger_type: LedgerType::Leaf,
            name: ArrayString::<256>::from(name).unwrap(),
            currency: None,
        };

        self.db.create(&account)
    }

    fn create_journal(
        &self,
        code: &'static str,
        name: &'static str,
        ledger_id: Option<LedgerId>,
    ) -> Result<journal::ActiveModel, OrmError> {
        let ledger_id = ledger_id.unwrap_or(self.ledger.id);
        let model = journal::Model {
            name: name.into(),
            code: code.into(),
            ledger_id,
        };

        self.db.create(&model)
    }

    fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: AccountId,
        account_cr_id: AccountId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        let journal_id: JournalId = journal_id.unwrap_or(self.journal.id);
        let model = journal_transaction::Model {
            journal_id,
            timestamp: timestamp(),
            state: TransactionState::Pending,
            amount: amount,
            description: desc.to_string(),
            posting_ref: None,
            account_dr_id,
            account_cr_id,
        };

        self.db.create(&model)
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
