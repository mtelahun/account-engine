use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::{
    domain::{ids::JournalId, JournalTransactionId, LedgerId, LedgerXactTypeCode, XactType},
    resource::{
        account_engine::AccountEngine, journal, ledger, ledger_xact_type, LedgerKey,
        LedgerPostingRef, TransactionState,
    },
    service::{GeneralJournalService, ServiceError},
    store::{memory::store::MemoryStore, postgres::store::PostgresStore, ResourceOperations},
    Store,
};

#[async_trait]
pub trait JournalTransactionService<R>: GeneralJournalService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
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
        > + Send
        + Sync
        + 'static,
{
    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        let ledger_xact_type = self.get_journal_entry_type(id).await?;

        let mut jxact_lines = <R as ResourceOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
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
        let sum_dr = dr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount);
        let sum_cr = cr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount);
        if sum_dr == Decimal::ZERO || sum_dr != sum_cr {
            return Err(ServiceError::Validation(format!(
                "the Dr and Cr sides of the transaction must be non-zero and equal: DR: {sum_dr}, CR: {sum_cr}"
            )));
        }

        let ledger_line: ledger::transaction::ledger::Model;
        let mut ledger_posted_list = Vec::<journal::transaction::general::line::ActiveModel>::new();
        let key = LedgerKey {
            ledger_id: cr_xact_lines[0].ledger_id,
            timestamp: cr_xact_lines[0].timestamp,
        };
        let entry = ledger::transaction::Model {
            ledger_id: key.ledger_id,
            timestamp: key.timestamp,
            ledger_xact_type_code: ledger_xact_type.code,
            amount: cr_xact_lines[0].amount,
            journal_ref: id,
        };
        let _ = self.store().insert(&entry).await?;
        let mut cr = *cr_xact_lines[0];
        cr.state = TransactionState::Posted;
        cr.posting_ref = Some(LedgerPostingRef {
            key,
            ledger_id: cr.ledger_id,
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
            dr.posting_ref = Some(LedgerPostingRef {
                key,
                ledger_id: dr.ledger_id,
            });
            ledger_posted_list.push(dr);
        }

        for line in jxact_lines.iter_mut() {
            for post_line in ledger_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.store()
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
