use account_engine::{
    domain::{
        entity::{
            external_account::account_id::AccountId,
            subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
        },
        journal_transaction::{JournalTransactionColumn, SpecialJournalTransaction},
        special_journal::special_journal_template_id::SpecialJournalTemplateId,
        LedgerAccount, ServiceError,
    },
    resource::{journal, subsidiary_ledger},
    shared_kernel::{ArrayString128, JournalTransactionId},
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::support::utils::timestamp;

use super::{
    service_test_interface::ServiceTestInterface, state_interface::StateInterface,
    test_column::TestColumn,
};

#[derive(Debug)]
pub struct TestSpecialJournalTransaction<S> {
    state: S,
    pub subsidiary: subsidiary_ledger::ActiveModel,
    pub journal: journal::ActiveModel,
    pub control: LedgerAccount,
    pub account_id: AccountId,
    pub template_id: SpecialJournalTemplateId,
    pub explanation: ArrayString128,
    pub xact_type_external_code: ExternalXactTypeCode,
    pub column1: JournalTransactionColumn,
    pub timestamp: NaiveDateTime,
    tx_template_columns: Vec<journal::transaction::special::template::column::ActiveModel>,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CreateLedgerType {
    #[default]
    Random,
    Ledger(LedgerAccount),
}

impl<S: StateInterface + ServiceTestInterface> TestSpecialJournalTransaction<S> {
    pub async fn new(state: S, control: CreateLedgerType, test_col: &TestColumn) -> Self {
        let timestamp = timestamp();
        let (sub, journal, control, account, tpl, tpl_col) =
            state.create_subsidiary("A/R", control).await;
        let column1 = journal::transaction::column::ledger_drcr::ActiveModel {
            journal_id: journal.id,
            timestamp,
            template_column_id: tpl_col[0].id,
            amount: test_col.amount(),
            ledger_dr_id: control.id(),
            ledger_cr_id: control.id(),
            column_total_id: None,
        };
        let column1 = JournalTransactionColumn::LedgerDrCr(column1);

        Self {
            state,
            subsidiary: sub,
            journal,
            control,
            account_id: account.id(),
            template_id: tpl.id,
            explanation: "Withdrew cash for lunch".into(),
            xact_type_external_code: "DF".into(),
            column1,
            timestamp,
            tx_template_columns: tpl_col,
        }
    }

    pub async fn journalize(
        &self,
    ) -> (
        SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
        Vec<JournalTransactionColumn>,
    ) {
        let lines = vec![self.column1];
        let (tx, tx_lines) = self
            .state
            .create_subsidiary_transaction(
                &self.journal.id,
                self.timestamp,
                &self.template_id,
                self.account_id,
                account_engine::shared_kernel::XactType::Dr,
                &"DF".into(),
                Decimal::from(100),
                &self.explanation,
                &self.tx_template_columns,
                &lines,
            )
            .await
            .expect("failed to create journalize subsidiary transaction");

        (tx, tx_lines)
    }

    pub async fn post(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        self.state.post_subsidiary_ledger(id).await?;
        Ok(self
            .state
            .post_general_ledger(self.journal.id, &vec![id])
            .await?)
    }

    pub async fn assert_column_match(&self, test_col: &TestColumn, is_posted: bool) {
        // Query SubLedgerService
        let records = self
            .state
            .get_subsidiary_transactions_by_journal(self.journal.id)
            .await
            .expect("failed to get journal transactions");
        let (jxacts, jx_columns) = &records[0];

        println!("jxact_lines: {:#?}", jx_columns);
        assert_eq!(
            records.len(),
            1,
            "One journal transaction was created in the subsidiary journal"
        );
        assert_eq!(
            jx_columns.len(),
            1,
            "The transaction has a single column, TWO ledger accounts"
        );

        for jx_column in jx_columns.iter() {
            assert_eq!(
                jx_column.journal_id(),
                self.journal.id,
                "Journal set properly"
            );
            assert_eq!(
                jx_column.timestamp(),
                self.timestamp,
                "Timestamp set properly"
            );
            assert_eq!(jx_column.amount(), test_col.amount(), "Amount set properly");
            if !is_posted {
                assert!(
                    !jx_column.posted(),
                    "journal column transaction state has not been posted"
                );
                assert!(
                    jx_column.column_total_id().is_none(),
                    "Pending transaction has no 'Total' entry"
                );
            } else {
                assert!(jx_column.posted(), "journal column transaction is 'Posted'");
                assert!(
                    jx_column.column_total_id().is_some(),
                    "Posted transaction has a 'Total' entry"
                );
            }
        }
        if is_posted {
            self.assert_match_column_posting_ref(jxacts.timestamp, &jx_columns)
                .await;
        }
    }

    async fn assert_match_column_posting_ref(
        &self,
        _timestamp: NaiveDateTime,
        _jx_columns: &Vec<JournalTransactionColumn>,
    ) {
        // for col in jx_columns.iter() {
        //     let col_total = self
        //         .state
        //         .get_column_total(col.id(), col.sequence)
        //         .await
        //         .expect("failed to get column total");
        //     let ledger_id_cr = self.tx_template_columns[0].cr_ledger_id;
        //     if ledger_id_cr.is_some() {
        //         let ledger_id = ledger_id_cr.unwrap();
        //         let expected: LedgerPostingRef = LedgerPostingRef::new(
        //             LedgerKey {
        //                 ledger_id: ledger_id,
        //                 timestamp: timestamp,
        //             },
        //             ledger_id,
        //         );
        //         assert_eq!(
        //             col_total.posting_ref_cr,
        //             Some(expected),
        //             "Cr PostingRef set properly"
        //         );
        //     } else {
        //         assert_eq!(
        //             col_total.posting_ref_cr, None,
        //             "If there is no Cr account then Cr PostingRef is None"
        //         );
        //     }
        //     let ledger_id_dr = self.tx_template_columns[0].dr_ledger_id;
        //     if ledger_id_dr.is_some() {
        //         let ledger_id = ledger_id_cr.unwrap();
        //         let expected: LedgerPostingRef = LedgerPostingRef::new(
        //             LedgerKey {
        //                 ledger_id: ledger_id,
        //                 timestamp: timestamp,
        //             },
        //             ledger_id_dr.unwrap(),
        //         );
        //         assert_eq!(
        //             col_total.posting_ref_dr,
        //             Some(expected),
        //             "Dr PostingRef set properly"
        //         );
        //     } else {
        //         assert_eq!(
        //             col_total.posting_ref_dr, None,
        //             "If there is no Dr account then Dr PostingRef is None"
        //         );
        //     }
        // }
        todo!()
    }

    pub async fn assert_posted(
        &self,
        _test_col: &TestColumn,
        id: JournalTransactionId,
        _posted: bool,
    ) {
        let jxact = self
            .state
            .get_subsidiary_transactions(Some(&vec![id]))
            .await
            .expect("failed to get journal transactions");
        let (_jxact, jx_lines) = &jxact[0];

        println!("jxact_lines: {:#?}", jx_lines);
        // assert!(
        //     posted,
        //     "subsidiary journal transaction posting returned success"
        // );

        for col in jx_lines {
            match col {
                JournalTransactionColumn::LedgerDrCr(_) => todo!(),
                JournalTransactionColumn::Text(_) => todo!(),
                JournalTransactionColumn::AccountDr(_) => todo!(),
                JournalTransactionColumn::AccountCr(_) => todo!(),
            }
        }

        // let posting_ref = jxact.posting_ref.unwrap();
        // let transaction = self
        //     .state
        //     .journal_entry_by_key(posting_ref.key())
        //     .await
        //     .expect("failed to get journal entry")
        //     .unwrap();
        // let transaction_account = self
        //     .state
        //     .get_journal_entry_transaction_account(&posting_ref)
        //     .await
        //     .expect("failed to get journal entry from subledger");
        // println!("transaction: {:#?}", transaction);
        // println!("transaction account: {:#?}", transaction_account);
        // assert_eq!(
        //     transaction_account.account_id, jxact.account_id,
        //     "Account Ids match"
        // );
        // assert_eq!(
        //     transaction_account.id(),
        //     posting_ref.key(),
        //     "Transaction keys match"
        // );
        // assert_eq!(
        //     transaction.ledger_xact_type_code,
        //     "LA".into(),
        //     "LedgerXactTypeCode is 'LA'"
        // );
        // assert_eq!(transaction.amount, test_col.amount, "Amounts match");
        // assert_eq!(
        //     transaction.journal_ref,
        //     jxact.id(),
        //     "transaction reference to journal transaction is correct."
        // )
    }
}
