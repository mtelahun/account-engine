use async_trait::async_trait;

use crate::{
    domain::{ids::JournalId, JournalTransactionId, LedgerId, LedgerXactTypeCode},
    infrastructure::data::db_context::postgres::PostgresStore,
    resource::{
        account_engine::AccountEngine, journal, ledger, ledger_xact_type, LedgerKey,
        LedgerPostingRef, TransactionState,
    },
    service::{GeneralJournalService, ServiceError},
    store::{memory::store::MemoryStore, ResourceOperations},
    Store,
};

#[async_trait]
pub trait JournalTransactionService<R>: GeneralJournalService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
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

        let jxact_lines = <R as ResourceOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), Some(&vec![id]))
        .await?;

        let key = LedgerKey {
            ledger_id: jxact_lines[0].cr_ledger_id,
            timestamp: jxact_lines[0].timestamp,
        };
        let entry = ledger::transaction::Model {
            ledger_id: key.ledger_id,
            timestamp: key.timestamp,
            ledger_xact_type_code: ledger_xact_type.code,
            amount: jxact_lines[0].amount,
            journal_ref: id,
        };
        let _ = self.store().insert(&entry).await?;
        let ledger_line = ledger::transaction::ledger::Model {
            ledger_id: key.ledger_id,
            timestamp: key.timestamp,
            ledger_dr_id: jxact_lines[0].dr_ledger_id,
        };
        let _ = self.store().insert(&ledger_line).await?;

        let mut line = jxact_lines[0];
        line.state = TransactionState::Posted;
        line.cr_posting_ref = Some(LedgerPostingRef {
            key,
            ledger_id: line.cr_ledger_id,
        });
        line.dr_posting_ref = Some(LedgerPostingRef {
            key,
            ledger_id: line.dr_ledger_id,
        });
        self.store()
            .update_journal_transaction_line_ledger_posting_ref(id, &line)
            .await?;

        Ok(true)
    }
}

impl JournalTransactionService<PostgresStore> for AccountEngine<PostgresStore> {}

impl JournalTransactionService<MemoryStore> for AccountEngine<MemoryStore> {}
