use async_trait::async_trait;

use crate::{
    domain::{ids::JournalId, JournalTransactionId, LedgerId, LedgerXactTypeCode, XactType},
    resource::{
        account_engine::AccountEngine, external, journal, ledger, ledger_xact_type, LedgerKey,
        PostingRef, TransactionState,
    },
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Store,
};

use super::{JournalService, ServiceError};

#[async_trait]
pub trait JournalTransactionService<R>: JournalService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::line::ledger::Model,
            journal::transaction::line::ledger::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::line::account::Model,
            journal::transaction::line::account::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
        + ResourceOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + ResourceOperations<
            ledger::transaction::account::Model,
            ledger::transaction::account::ActiveModel,
            LedgerKey,
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + ResourceOperations<external::account::Model, external::account::ActiveModel, LedgerId>
        + Send
        + Sync
        + 'static,
{
    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        let ledger_xact_type = self.get_journal_entry_type(id).await?;

        let mut jxact_lines = <R as ResourceOperations<
            journal::transaction::line::ledger::Model,
            journal::transaction::line::ledger::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), Some(&vec![id]))
        .await?;
        let mut jxact_lines2 = <R as ResourceOperations<
            journal::transaction::line::account::Model,
            journal::transaction::line::account::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), Some(&vec![id]))
        .await?;
        let cr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        let cr_xact_lines2 = jxact_lines2
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines2 = jxact_lines2
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        let sum_dr = dr_xact_lines.len() + dr_xact_lines2.len();
        let sum_cr = cr_xact_lines.len() + cr_xact_lines2.len();
        if sum_dr == 0 || sum_dr != sum_cr {
            return Err(ServiceError::Validation(
                "the Dr and Cr sides of the transaction must be equal".to_string(),
            ));
        }
        let key: LedgerKey;
        let entry: ledger::transaction::Model;
        let ledger_line: ledger::transaction::ledger::Model;
        let account_line: ledger::transaction::account::Model;
        let mut ledger_posted_list = Vec::<journal::transaction::line::ledger::ActiveModel>::new();
        let mut account_posted_list =
            Vec::<journal::transaction::line::account::ActiveModel>::new();
        if !cr_xact_lines.is_empty() {
            key = LedgerKey {
                ledger_id: cr_xact_lines[0].ledger_id,
                timestamp: cr_xact_lines[0].timestamp,
            };
            entry = ledger::transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: cr_xact_lines[0].amount,
                journal_ref: id,
            };
            let _ = self.store().insert(&entry).await?;
            let mut cr = *cr_xact_lines[0];
            cr.state = TransactionState::Posted;
            cr.posting_ref = Some(PostingRef {
                key,
                account_id: cr.ledger_id,
            });
            ledger_posted_list.push(cr);
            if !dr_xact_lines.is_empty() {
                ledger_line = ledger::transaction::ledger::Model {
                    ledger_id: key.ledger_id,
                    timestamp: key.timestamp,
                    ledger_dr_id: dr_xact_lines[0].ledger_id,
                };
                let _ = self.store().insert(&ledger_line).await?;
                let mut dr = *dr_xact_lines[0];
                dr.state = TransactionState::Posted;
                dr.posting_ref = Some(PostingRef {
                    key,
                    account_id: dr.ledger_id,
                });
                ledger_posted_list.push(dr);
            } else {
                account_line = ledger::transaction::account::Model {
                    account_id: dr_xact_lines2[0].account_id,
                    ledger_id: key.ledger_id,
                    timestamp: key.timestamp,
                    xact_type_code: XactType::Dr,
                    xact_type_external_code: dr_xact_lines2[0].xact_type_external.unwrap(),
                };
                let _ = self.store().insert(&account_line).await?;
                let mut dr = *dr_xact_lines2[0];
                dr.state = TransactionState::Posted;
                dr.posting_ref = Some(PostingRef {
                    key,
                    account_id: dr.account_id,
                });
                account_posted_list.push(dr);
            }
        } else {
            key = LedgerKey {
                ledger_id: dr_xact_lines[0].ledger_id,
                timestamp: dr_xact_lines[0].timestamp,
            };
            entry = ledger::transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: dr_xact_lines[0].amount,
                journal_ref: id,
            };
            account_line = ledger::transaction::account::Model {
                account_id: cr_xact_lines2[0].account_id,
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                xact_type_code: XactType::Dr,
                xact_type_external_code: dr_xact_lines2[0].xact_type_external.unwrap(),
            };
            let _ = self.store().insert(&entry).await?;
            let _ = self.store().insert(&account_line).await?;
            let mut dr = *dr_xact_lines[0];
            dr.state = TransactionState::Posted;
            dr.posting_ref = Some(PostingRef {
                key,
                account_id: dr.ledger_id,
            });
            ledger_posted_list.push(dr);
            let mut cr = *cr_xact_lines2[0];
            cr.state = TransactionState::Posted;
            cr.posting_ref = Some(PostingRef {
                key,
                account_id: cr.account_id,
            });
            account_posted_list.push(cr);
        }
        // for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
        //     let key = LedgerKey {
        //         ledger_id: cr.ledger_id,
        //         timestamp: cr.timestamp,
        //     };
        //     let entry = ledger::transaction::Model {
        //         ledger_id: key.ledger_id,
        //         timestamp: key.timestamp,
        //         ledger_xact_type_code: ledger_xact_type.code,
        //         amount: cr.amount,
        //         journal_ref: id,
        //     };
        //     let tx_dr = ledger::transaction::ledger::Model {
        //         ledger_id: key.ledger_id,
        //         timestamp: key.timestamp,
        //         ledger_dr_id: dr.ledger_id,
        //     };

        //     let _ = self.store().insert(&entry).await?;
        //     let _ = self.store().insert(&tx_dr).await?;
        //     let mut cr = *cr;
        //     cr.state = TransactionState::Posted;
        //     cr.posting_ref = Some(PostingRef {
        //         key,
        //         account_id: cr.ledger_id,
        //     });
        //     let mut dr = *dr;
        //     dr.state = TransactionState::Posted;
        //     dr.posting_ref = Some(PostingRef {
        //         key,
        //         account_id: dr.ledger_id,
        //     });
        //     ledger_posted_list.push(dr);
        //     ledger_posted_list.push(cr);
        // }

        for line in jxact_lines.iter_mut() {
            for post_line in ledger_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.store()
                        .update_journal_transaction_line_ledger_posting_ref(id, post_line)
                        .await?;
                }
            }
        }
        for line in jxact_lines2.iter_mut() {
            for post_line in account_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.store()
                        .update_journal_transaction_line_account_posting_ref(id, post_line)
                        .await?;
                }
            }
        }

        Ok(true)
    }
}

impl JournalTransactionService<PostgresStore> for AccountEngine<PostgresStore> {}

impl JournalTransactionService<MemoryStore> for AccountEngine<MemoryStore> {}
