use account_engine::{
    application::{
        error::ServiceError, general_journal::GeneralJournalService,
        general_ledger::GeneralLedgerService, journal_transaction::JournalTransactionService,
        ledger::LedgerService, period::AccountingPeriodService,
        special_journal::SpecialJournalService,
        special_journal_transaction::SpecialJournalTransactionService,
    },
    domain::{
        entity::{ledger::ledger_id::LedgerId, xact_type::XactType},
        LedgerAccount,
    },
    infrastructure::persistence::context::error::OrmError,
    resource::{
        accounting_period,
        journal::{
            self,
            transaction::{AccountPostingRef, JournalTransactionColumnType},
        },
        InterimType, LedgerType, TransactionState,
    },
};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::support::{
    memstore::state::{state_with_memstore, TestState},
    utils::timestamp,
};

#[tokio::test]
async fn non_existant_ledger() {
    // Arrange
    let state = state_with_memstore().await;
    let ledger_id = LedgerId::new();

    // Act
    let res = state
        .engine
        .get_ledgers(Some(&vec![ledger_id]))
        .await
        .unwrap();

    // Assert
    let all: Vec<LedgerAccount> = GeneralLedgerService::get_ledgers(&state.engine, None)
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
    let state = state_with_memstore().await;

    // Act
    let _ = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Assets",
            "1000",
            None,
        )
        .await
        .expect("1st ledger creation failed");
    let assets_same_gl = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Assets",
            "1000",
            None,
        )
        .await;

    // Assert
    assert!(assets_same_gl.is_err(), "failed to create second account");
    assert_eq!(
        assets_same_gl.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::DuplicateResource(OrmError::DuplicateRecord(
            "account 1000".into(),
        )))
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
async fn parent_is_intermediate() {
    // Arrange
    let state = state_with_memstore().await;

    // Act
    let assets = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Assets",
            "1000",
            None,
        )
        .await
        .unwrap();
    let cash = state
        .engine
        .create_ledger(LedgerType::Leaf, assets.id(), "Cash", "1001", None)
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
    let state = state_with_memstore().await;
    let assets_original = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Assets",
            "1000",
            None,
        )
        .await
        .expect("first account created successfully");

    // Act
    let assets_same_gl = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Assets",
            "1001",
            None,
        )
        .await
        .expect("second account with same name created successfully");

    // Assert
    assert_eq!(
        assets_original.name(),
        assets_same_gl.name(),
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
    let state = state_with_memstore().await;
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
    let state = state_with_memstore().await;
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
    let state = state_with_memstore().await;
    let j1 = journal::Model {
        name: "General".into(),
        code: "S".into(),
        ..Default::default()
    };
    let j2 = journal::Model {
        name: "Sales".into(),
        code: "S".into(),
        ..Default::default()
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
    let state = state_with_memstore().await;
    let cash1 = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Cash",
            "1001",
            None,
        )
        .await
        .unwrap();
    let bank1 = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Bank",
            "1002",
            None,
        )
        .await
        .unwrap();

    let now = timestamp();
    let jx1_line1 = journal::transaction::general::line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        dr_ledger_id: cash1.id(),
        cr_ledger_id: bank1.id(),
        amount: Decimal::ZERO,
        ..Default::default()
    };
    let jx1 = journal::transaction::general::Model {
        journal_id: state.journal.id,
        timestamp: now,
        explanation: "Withdrew cash for lunch".into(),
        lines: vec![jx1_line1],
    };
    let jx_same_ledger = jx1.clone();

    // Act
    state
        .engine
        .create_general_transaction(&jx1)
        .await
        .expect("failed to create 1st journal transaction");
    let same_ledger_tx = state
        .engine
        .create_general_transaction(&jx_same_ledger)
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
    let fake_account_id = LedgerId::new();
    let state = state_with_memstore().await;
    let bank = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Bank",
            "1002",
            None,
        )
        .await
        .unwrap();
    let jxact = SimpleJournalTransaction::new(&state);
    let invalid_cases = [
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.dr_ledger_id = fake_account_id;
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1];
                jx1
            },
            ServiceError::EmptyRecord(format!("ledger id: {fake_account_id}",)),
            "line1: dr_ledger_id is fake",
        ),
        (
            {
                let mut jx_line1 = jxact.line1.clone();
                jx_line1.dr_ledger_id = bank.id();
                jx_line1.cr_ledger_id = fake_account_id;
                let mut jx1 = jxact.jx.clone();
                jx1.lines = vec![jx_line1];
                jx1
            },
            ServiceError::EmptyRecord(format!("ledger id: {fake_account_id}",)),
            "line1: cr_ledger_id is fake",
        ),
        // (
        //     {
        //         let mut jx_line1 = jxact.line1.clone();
        //         jx_line1.account_id = fake_account_id;
        //         let mut jx_line2 = jxact.line2.clone();
        //         jx_line2.ledger_id = bank.id;
        //         let mut jx1 = jxact.jx.clone();
        //         jx1.lines = vec![jx_line1, jx_line2];
        //         jx1
        //     },
        //     ServiceError::EmptyRecord(format!(
        //         "ledger id: {fake_account_id}",
        //     )),
        //     "line1: account_id is fake",
        // ),
        // (
        //     {
        //         let mut jx_line1 = jxact.line1.clone();
        //         jx_line1.ledger_id = bank.id;
        //         let mut jx_line2 = jxact.line2.clone();
        //         jx_line2.account_id = fake_account_id;
        //         let mut jx1 = jxact.jx.clone();
        //         jx1.lines = vec![jx_line1, jx_line2];
        //         jx1
        //     },
        //     ServiceError::EmptyRecord(format!(
        //         "ledger id: {fake_account_id}",
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
    jx1: &journal::transaction::general::Model,
    expected_error: ServiceError,
    msg: &str,
) {
    // Act
    let jx1_db = state.engine.create_general_transaction(jx1).await;

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
    let state = state_with_memstore().await;
    let cash = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Cash",
            "1001",
            None,
        )
        .await
        .expect("failed to creae Cash account");
    let bank = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Bank",
            "1002",
            None,
        )
        .await
        .expect("failed to create Bank account");
    let jxact = state
        .create_journal_xact(
            Decimal::from(100),
            cash.id(),
            bank.id(),
            "Withdrew cash",
            None,
        )
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
        .journal_entries(bank.id())
        .await
        .expect("failed to get journal enries for Bank ledger");
    let cash_entries = state
        .engine
        .journal_entries(cash.id())
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
        .journal_entry_by_posting_ref(jxact.lines[0].dr_posting_ref.unwrap())
        .await
        .expect("failed to fech journal entry #1 by posting ref.")
        .expect("journal entry #1 is empty");
    let entry2 = state
        .engine
        .journal_entry_by_posting_ref(jxact.lines[0].cr_posting_ref.unwrap())
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
    let cr_account = state
        .engine
        .get_ledgers(Some(&vec![bank.id()]))
        .await
        .expect("unexpected search error")[0];
    let dr_account = state
        .engine
        .get_ledgers(Some(&vec![cash.id()]))
        .await
        .expect("unexpected search error")[0];
    assert_eq!(
        cr_account.id(),
        jxact.lines[0].cr_ledger_id,
        "ledger CR ac. matches journal"
    );
    assert_eq!(
        dr_account.id(),
        jxact.lines[0].dr_ledger_id,
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
        jxact.lines[0].dr_ledger_id, jxact.lines[0].cr_ledger_id,
        "journal transaction dr and cr accounts are different"
    );
}

#[tokio::test]
async fn post_journal_transaction_unbalanced() {
    // Arrange
    let now = timestamp();
    let state = state_with_memstore().await;
    let bank = state
        .engine
        .create_ledger(
            LedgerType::Leaf,
            state.general_ledger.root,
            "Bank",
            "1002",
            None,
        )
        .await
        .expect("failed to create Bank account");
    let cr_line = journal::transaction::general::line::Model {
        journal_id: state.journal.id,
        timestamp: now,
        cr_ledger_id: bank.id(),
        amount: Decimal::ONE,
        ..Default::default()
    };
    let model = journal::transaction::general::Model {
        journal_id: state.journal.id,
        timestamp: now,
        explanation: "".into(),
        lines: vec![cr_line],
    };

    // Act
    let jxact = state.engine.create_general_transaction(&model).await;

    // Assert
    assert!(jxact.is_err(), "posting an unbalanced journal tx fails");
    assert_eq!(
        jxact.err().unwrap(),
        Err::<(), ServiceError>(ServiceError::EmptyRecord(
            "ledger id: 00000000-0000-0000-0000-000000000000".into(),
        ))
        .err()
        .unwrap()
    );
}

#[tokio::test]
async fn special_journal_dr_account_plus_one_double_ledger_column() {
    // Arrange
    // Account | Ledger Dr/Cr |
    // --------|-----------|
    //   abc   |  100.00   |
    let state = state_with_memstore().await;
    let (_subledger, jrn, _ledger, acct, tpl, tpl_col) = state
        .create_subsidiary_ledger("Subsidiary", XactType::Dr)
        .await;

    // Act 1
    let (stx, stx_cols) = state
        .create_special_transaction(
            &jrn,
            &tpl.id,
            acct.id(),
            XactType::Dr,
            Decimal::from(100),
            &tpl_col,
        )
        .await
        .expect("failed to create special journal transaction");

    // Assert 1
    assert_eq!(
        stx.journal_id, jrn.id,
        "transaction journal_id matches input"
    );
    assert_eq!(
        stx_cols[0].amount(),
        Decimal::from(100),
        "special journal account amount is 100.00"
    );
    assert_eq!(
        stx_cols[0].amount(),
        stx_cols[1].amount(),
        "special journal account and ledger amounts are equal"
    );
    assert_eq!(
        stx_cols[0].account_id().unwrap(),
        acct.id(),
        "special journal account is set correctly"
    );
    assert_eq!(
        stx_cols[1].ledger_cr_id().unwrap(),
        tpl_col[1].cr_ledger_id.unwrap(),
        "special journal column ledger Cr side identical to template column"
    );
    assert_eq!(
        stx_cols[1].ledger_dr_id().unwrap(),
        tpl_col[1].dr_ledger_id.unwrap(),
        "special journal column ledger Dr side identical to template column"
    );

    // Act 2
    let res = SpecialJournalTransactionService::post_to_account(&state.engine, stx.id())
        .await
        .expect("failed to post transaction to accounts");
    let records =
        SpecialJournalService::get_special_transactions(&state.engine, Some(&vec![stx.id()]))
            .await
            .expect("failed to retrieve transaction posted to accounts");
    let (stx, stx_cols) = &records[0];

    // Assert 2
    assert!(res, "post to account succeeded");
    assert_eq!(stx_cols.len(), 2, "there are 2 columns in the transaction");
    assert_eq!(
        stx_cols[0].column_type(),
        JournalTransactionColumnType::AccountDr,
        "the column is a Dr Account"
    );
    assert!(
        stx_cols[0].posted(),
        "special transaction was posted to the account"
    );
    assert_eq!(
        stx_cols[0].account_posting_ref(),
        Some(AccountPostingRef::new(&acct.id(), stx.timestamp)),
        "the account contains a posting reference"
    );
}

#[tokio::test]
async fn special_journal_cr_account_plus_one_double_ledger_column() {
    // Arrange
    // Account | Ledger Dr/Cr |
    // --------|-----------|
    //   abc   |  100.00   |
    let state = state_with_memstore().await;
    let (_subledger, jrn, _ledger, acct, tpl, tpl_col) = state
        .create_subsidiary_ledger("Subsidiary", XactType::Cr)
        .await;

    // Act 1
    let (stx, stx_cols) = state
        .create_special_transaction(
            &jrn,
            &tpl.id,
            acct.id(),
            XactType::Cr,
            Decimal::from(100),
            &tpl_col,
        )
        .await
        .expect("failed to create special journal transaction");

    // Assert 1
    assert_eq!(
        stx.journal_id, jrn.id,
        "transaction journal_id matches input"
    );
    assert_eq!(
        stx_cols[0].amount(),
        Decimal::from(100),
        "special journal account amount is 100.00"
    );
    assert_eq!(
        stx_cols[0].amount(),
        stx_cols[1].amount(),
        "special journal account and ledger amounts are equal"
    );
    assert_eq!(
        stx_cols[0].account_id().unwrap(),
        acct.id(),
        "special journal account is set correctly"
    );
    assert_eq!(
        stx_cols[1].ledger_cr_id().unwrap(),
        tpl_col[1].cr_ledger_id.unwrap(),
        "special journal column ledger Cr side identical to template column"
    );
    assert_eq!(
        stx_cols[1].ledger_dr_id().unwrap(),
        tpl_col[1].dr_ledger_id.unwrap(),
        "special journal column ledger Dr side identical to template column"
    );

    // Act 2
    let res = SpecialJournalTransactionService::post_to_account(&state.engine, stx.id())
        .await
        .expect("failed to post transaction to accounts");
    let records =
        SpecialJournalService::get_special_transactions(&state.engine, Some(&vec![stx.id()]))
            .await
            .expect("failed to retrieve transaction posted to accounts");
    let (stx, stx_cols) = &records[0];

    // Assert 2
    assert!(res, "post to account succeeded");
    assert_eq!(stx_cols.len(), 2, "there are 2 columns in the transaction");
    assert_eq!(
        stx_cols[0].column_type(),
        JournalTransactionColumnType::AccountCr,
        "the column is a Cr Account"
    );
    assert!(
        stx_cols[0].posted(),
        "special transaction was posted to the account"
    );
    assert_eq!(
        stx_cols[0].account_posting_ref(),
        Some(AccountPostingRef::new(&acct.id(), stx.timestamp)),
        "the account contains a posting reference"
    );
}

// #[tokio::test]
// async fn subsidiary_journal_tx_one_column_one_ledger() {
//     // Arrange
//     // Account | Ledger Dr |
//     // --------|-----------|
//     //         |  100.00   |
//     let state = state_with_memstore().await;
//     let column = TestColumn::new_ledger_drcr(
//         &state,
//         CreateLedgerType::Random,
//         CreateLedgerType::Random,
//         Decimal::ONE_HUNDRED,
//     )
//     .await;
//     let jxact = SpecialJournalTransaction::new(state, CreateLedgerType::Random, &column).await;

//     // Act
//     let (tx, _) = jxact.journalize().await;

//     // Assert
//     jxact.assert_column_match(&column, false).await;

//     // Act
//     let posted = jxact.post(tx.id()).await;

//     // Assert
//     assert!(
//         posted.is_err(),
//         "A special journal with only ONE column AND ONE account is unbalanced and must fail"
//     );
//     assert_eq!(
//         posted.unwrap_err(),
//         ServiceError::Validation(
//             "the Dr and Cr sides of the transaction must be equal".to_string()
//         ),
//         "The unbalanced transaction error is correctly returned"
//     );
// }

// #[tokio::test]
// async fn subsidiary_journal_tx_one_column_two_ledger() {
//     // Arrange
//     //            | Ledger Dr/ |
//     // Account Dr | Ledger Cr  |
//     // -----------|------------|
//     //            |  100.00    |
//     let state = state_with_memstore().await;
//     let receivables = state.create_ledger_leaf().await;
//     let column = TestColumn::new_ledger_drcr(
//         &state,
//         CreateLedgerType::Ledger(receivables),
//         CreateLedgerType::Random,
//         Decimal::ONE_HUNDRED,
//     )
//     .await;
//     let jxact =
//         SpecialJournalTransaction::new(state, CreateLedgerType::Ledger(receivables), &column).await;

//     // Act
//     let (tx, _) = jxact.journalize().await;

//     // Assert
//     jxact.assert_column_match(&column, false).await;

//     // Act
//     let posted = jxact
//         .post(tx.id())
//         .await
//         .expect("failed to post subsidiary transaction");

//     // Assert
//     jxact.assert_posted(&column, tx.id(), posted).await;
//     jxact.assert_column_match(&column, true).await;
// }

#[derive(Debug)]
struct SimpleJournalTransaction {
    jx: journal::transaction::general::Model,
    line1: journal::transaction::general::line::Model,
    _timestamp: NaiveDateTime,
}

impl SimpleJournalTransaction {
    pub fn new(state: &TestState) -> Self {
        let timestamp = timestamp();
        let line1 = journal::transaction::general::line::Model {
            journal_id: state.journal.id,
            timestamp,
            amount: Decimal::ONE,
            ..Default::default()
        };
        let jx = journal::transaction::general::Model {
            journal_id: state.journal.id,
            timestamp,
            explanation: "Withdrew cash for lunch".into(),
            lines: Vec::<journal::transaction::general::line::Model>::new(),
        };

        Self {
            jx,
            line1,
            _timestamp: timestamp,
        }
    }

    pub async fn _journalize(
        &self,
        state: &TestState,
        desc: &str,
    ) -> journal::transaction::general::ActiveModel {
        let model = journal::transaction::general::Model {
            journal_id: self.line1.journal_id,
            timestamp: self._timestamp,
            explanation: desc.into(),
            lines: vec![self.line1],
        };

        state
            .engine
            .create_general_transaction(&model)
            .await
            .expect(format!("failed to create simple journal transaction: {desc}").as_str())
    }
}
