use account_engine::{
    domain::{JournalTransactionId, LedgerId},
    resource::{journal, ledger, subsidiary_ledger, LedgerKey, LedgerPostingRef, TransactionState},
    service::{
        LedgerService, ServiceError, SubsidiaryJournalService, SubsidiaryJournalTransactionService,
        SubsidiaryLedgerService,
    },
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{support::state::TestState, support::utils::timestamp};

#[derive(Debug)]
pub struct SpecialJournalTransaction {
    pub subsidiary: subsidiary_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
    pub control: ledger::ActiveModel,
    pub xact: journal::transaction::special::Model,
    pub column1: journal::transaction::special::column::Model,
    pub timestamp: NaiveDateTime,
    tx_template_columns: Vec<journal::transaction::special::template::column::ActiveModel>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TestColumn {
    ledger_dr: Option<ledger::ActiveModel>,
    ledger_cr: Option<ledger::ActiveModel>,
    amount: Decimal,
    sequence: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CreateLedgerType {
    #[default]
    None,
    Random,
    Ledger(ledger::ActiveModel),
}

impl SpecialJournalTransaction {
    pub async fn new(state: &TestState, control: CreateLedgerType, test_col: &TestColumn) -> Self {
        let timestamp = timestamp();
        let (sub, journal, control, account, _, tx_template_columns) =
            state.create_subsidiary("A/R", control).await;
        let column1 = journal::transaction::special::column::Model {
            journal_id: journal.id,
            timestamp,
            sequence: 1,
            dr_ledger_id: test_col.dr_ledger_id(),
            cr_ledger_id: test_col.cr_ledger_id(),
            amount: test_col.amount(),
            ..Default::default()
        };
        let jx = journal::transaction::special::Model {
            journal_id: journal.id,
            timestamp,
            explanation: "Withdrew cash for lunch".into(),
            account_id: account.id,
            xact_type_external: Some("DF".into()),
            ..Default::default()
        };

        Self {
            subsidiary: sub,
            journal,
            control,
            xact: jx,
            column1,
            timestamp,
            tx_template_columns,
        }
    }

    pub async fn journalize(
        &self,
        state: &TestState,
    ) -> (
        journal::transaction::special::ActiveModel,
        Vec<journal::transaction::special::column::ActiveModel>,
    ) {
        let lines = vec![self.column1];
        let tx = state
            .engine
            .create_subsidiary_transaction(&self.xact, &lines)
            .await
            .expect("failed to create journalize subsidiary transaction");
        let tx_lines = state
            .engine
            .get_subsidiary_transaction_columns(Some(&vec![tx.id()]))
            .await
            .expect("failed to get lines of journalized transaction");

        (tx, tx_lines)
    }

    pub async fn post(
        &self,
        state: &TestState,
        id: JournalTransactionId,
    ) -> Result<bool, ServiceError> {
        state.engine.post_subsidiary_ledger(id).await?;
        Ok(state
            .engine
            .post_general_ledger(self.journal.id, &vec![id])
            .await?)
    }

    pub async fn assert_column_match(
        &self,
        state: &TestState,
        test_col: &TestColumn,
        is_posted: bool,
    ) {
        // Query SubLedgerService
        let jxacts = state
            .engine
            .get_subsidiary_transactions_by_journal(self.journal.id)
            .await
            .expect("failed to get journal transactions");
        let jx_columns = state
            .engine
            .get_subsidiary_transaction_columns(Some(&vec![jxacts[0].id()]))
            .await
            .expect("failed to get subsidiary transaction lines");

        println!("jxact_lines: {:#?}", jx_columns);
        assert_eq!(
            jxacts.len(),
            1,
            "One journal transaction was created in the subsidiary journal"
        );
        if test_col.cr_ledger_id().is_some() && test_col.dr_ledger_id().is_some() {
            assert_eq!(
                jx_columns.len(),
                1,
                "The transaction has a single column, TWO ledger accounts"
            );
        } else {
            assert_eq!(
                jx_columns.len(),
                1,
                "The transaction has a single column, ONE ledger account"
            );
        }
        for jx_column in jx_columns.iter() {
            assert_eq!(
                jx_column.journal_id, self.journal.id,
                "Journal set properly"
            );
            assert_eq!(
                jx_column.timestamp, self.xact.timestamp,
                "Timestamp set properly"
            );
            assert_eq!(
                jx_column.sequence,
                test_col.sequence(),
                "sequence set properly"
            );
            assert_eq!(jx_column.amount, test_col.amount(), "Amount set properly");
            if !is_posted {
                assert_eq!(
                    jx_column.state,
                    TransactionState::Pending,
                    "journal column transaction state is 'Pending'"
                );
                assert!(
                    jx_column.column_total_id.is_none(),
                    "Pending transaction has no 'Total' entry"
                );
            } else {
                assert_eq!(
                    jx_column.state,
                    TransactionState::Posted,
                    "journal column transaction state is 'Posted'"
                );
                assert!(
                    jx_column.column_total_id.is_some(),
                    "Posted transaction has a 'Total' entry"
                );
            }
            if test_col.cr_ledger_id().is_some() {
                assert_eq!(
                    jx_column.cr_ledger_id,
                    Some(test_col.ledger_cr.unwrap().id),
                    "Cr ledger set properly"
                );
            }
            if test_col.dr_ledger_id().is_some() {
                assert_eq!(
                    jx_column.dr_ledger_id,
                    Some(test_col.ledger_dr.unwrap().id),
                    "Dr ledger set properly"
                );
            }
        }
        if is_posted {
            self.assert_match_column_posting_ref(state, jxacts[0].timestamp, &jx_columns)
                .await;
        }
    }

    async fn assert_match_column_posting_ref(
        &self,
        state: &TestState,
        timestamp: NaiveDateTime,
        jx_columns: &Vec<journal::transaction::special::column::ActiveModel>,
    ) {
        for col in jx_columns.iter() {
            let col_total = state
                .engine
                .get_column_total(col.id(), col.sequence)
                .await
                .expect("failed to get column total");
            let ledger_id_cr = self.tx_template_columns[0].cr_ledger_id;
            if ledger_id_cr.is_some() {
                let ledger_id = ledger_id_cr.unwrap();
                let expected: LedgerPostingRef = LedgerPostingRef::new(
                    LedgerKey {
                        ledger_id: ledger_id,
                        timestamp: timestamp,
                    },
                    ledger_id,
                );
                assert_eq!(
                    col_total.posting_ref_cr,
                    Some(expected),
                    "Cr PostingRef set properly"
                );
            } else {
                assert_eq!(
                    col_total.posting_ref_cr, None,
                    "If there is no Cr account then Cr PostingRef is None"
                );
            }
            let ledger_id_dr = self.tx_template_columns[0].dr_ledger_id;
            if ledger_id_dr.is_some() {
                let ledger_id = ledger_id_cr.unwrap();
                let expected: LedgerPostingRef = LedgerPostingRef::new(
                    LedgerKey {
                        ledger_id: ledger_id,
                        timestamp: timestamp,
                    },
                    ledger_id_dr.unwrap(),
                );
                assert_eq!(
                    col_total.posting_ref_dr,
                    Some(expected),
                    "Dr PostingRef set properly"
                );
            } else {
                assert_eq!(
                    col_total.posting_ref_dr, None,
                    "If there is no Dr account then Dr PostingRef is None"
                );
            }
        }
    }

    pub async fn assert_posted(
        &self,
        state: &TestState,
        test_col: &TestColumn,
        id: JournalTransactionId,
        posted: bool,
    ) {
        let jxact = state
            .engine
            .get_subsidiary_transactions(Some(&vec![id]))
            .await
            .expect("failed to get journal transactions");
        let jxact = jxact[0];
        let jx_lines = state
            .engine
            .get_subsidiary_transaction_columns(Some(&vec![id]))
            .await
            .expect("failed to get subsidiary transaction lines");

        println!("jxact_lines: {:#?}", jx_lines);
        assert!(
            posted,
            "subsidiary journal transaction posting returned success"
        );
        assert_eq!(
            jxact.account_posted_state,
            TransactionState::Posted,
            "The transaction has been posted to the external account"
        );

        let posting_ref = jxact.posting_ref.unwrap();
        let transaction = state
            .engine
            .journal_entry_by_key(posting_ref.key())
            .await
            .expect("failed to get journal entry")
            .unwrap();
        let transaction_account = state
            .engine
            .get_journal_entry_transaction_account(&posting_ref)
            .await
            .expect("failed to get journal entry from subledger");
        println!("transaction: {:#?}", transaction);
        println!("transaction account: {:#?}", transaction_account);
        assert_eq!(
            transaction_account.account_id, jxact.account_id,
            "Account Ids match"
        );
        assert_eq!(
            transaction_account.id(),
            posting_ref.key(),
            "Transaction keys match"
        );
        assert_eq!(
            transaction.ledger_xact_type_code,
            "LA".into(),
            "LedgerXactTypeCode is 'LA'"
        );
        assert_eq!(transaction.amount, test_col.amount, "Amounts match");
        assert_eq!(
            transaction.journal_ref,
            jxact.id(),
            "transaction reference to journal transaction is correct."
        )
    }
}

impl TestColumn {
    pub async fn new(
        state: &TestState,
        sequence: usize,
        use_dr: CreateLedgerType,
        use_cr: CreateLedgerType,
        amount: Decimal,
    ) -> Self {
        let ledger_dr = match use_dr {
            CreateLedgerType::None => None,
            CreateLedgerType::Random => Some(state.create_ledger_leaf().await),
            CreateLedgerType::Ledger(l) => Some(l),
        };
        let ledger_cr = match use_cr {
            CreateLedgerType::None => None,
            CreateLedgerType::Random => Some(state.create_ledger_leaf().await),
            CreateLedgerType::Ledger(l) => Some(l),
        };
        Self {
            ledger_dr,
            ledger_cr,
            amount,
            sequence,
            ..Default::default()
        }
    }

    pub fn cr_ledger_id(&self) -> Option<LedgerId> {
        match self.ledger_cr {
            Some(ledger) => Some(ledger.id),
            None => None,
        }
    }

    pub fn dr_ledger_id(&self) -> Option<LedgerId> {
        match self.ledger_dr {
            Some(ledger) => Some(ledger.id),
            None => None,
        }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn sequence(&self) -> usize {
        self.sequence
    }
}
