use std::iter::zip;

use async_trait::async_trait;

use crate::{
    domain::{ids::JournalId, AccountId, JournalTransactionId, LedgerXactTypeCode, XactType},
    resource::{
        account_engine::AccountEngine, journal, ledger, ledger_xact_type, LedgerKey, PostingRef,
        TransactionState,
    },
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Repository,
};

use super::{JournalService, ServiceError};

#[async_trait]
pub trait JournalTransactionService<R>: JournalService<R>
where
    R: Repository
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
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
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + Send
        + Sync
        + 'static,
{
    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        let ledger_xact_type = self.get_journal_entry_type(id).await?;

        let mut jxact_lines = <R as ResourceOperations<
            journal::transaction::line::ledger::Model,
            journal::transaction::line::ledger::ActiveModel,
            JournalTransactionId,
        >>::get(self.repository(), Some(&vec![id]))
        .await?;
        let cr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        let mut ledger_posted_list = Vec::<journal::transaction::line::ledger::ActiveModel>::new();
        for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
            let key = LedgerKey {
                ledger_id: cr.ledger_id,
                timestamp: cr.timestamp,
            };
            let entry = ledger::transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: cr.amount,
                journal_ref: id,
            };
            let tx_dr = ledger::transaction::ledger::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_dr_id: dr.ledger_id,
            };

            let _ = self.repository().insert(&entry).await?;
            let _ = self.repository().insert(&tx_dr).await?;
            let mut cr = *cr;
            cr.state = TransactionState::Posted;
            cr.posting_ref = Some(PostingRef {
                key,
                account_id: cr.ledger_id,
            });
            let mut dr = *dr;
            dr.state = TransactionState::Posted;
            dr.posting_ref = Some(PostingRef {
                key,
                account_id: dr.ledger_id,
            });
            ledger_posted_list.push(dr);
            ledger_posted_list.push(cr);
        }

        for line in jxact_lines.iter_mut() {
            for post_line in ledger_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.repository()
                        .update_journal_transaction_line_ledger_posting_ref(id, post_line)
                        .await?;
                }
            }
        }

        Ok(true)
    }
}

impl JournalTransactionService<PostgresStore> for AccountEngine<PostgresStore> {}

impl JournalTransactionService<MemoryStore> for AccountEngine<MemoryStore> {}
