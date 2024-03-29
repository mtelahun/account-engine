use async_trait::async_trait;

use crate::{
    domain::{
        entity::{
            general_journal::journal_id::JournalId,
            general_journal_transaction::journal_transaction_id::JournalTransactionId,
            ledger::ledger_id::LedgerId, ledger_xact_type_code::LedgerXactTypeCode,
        },
        GeneralJournalService, ServiceError,
    },
    infrastructure::persistence::context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{
        account_engine::AccountEngine, journal, ledger, ledger_xact_type, LedgerKey,
        LedgerPostingRef, TransactionState,
    },
    Store,
};

#[async_trait]
pub trait JournalTransactionService<R>: GeneralJournalService<R>
where
    R: Store
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            ledger::transaction::Model,
            ledger::transaction::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger::transaction::account::Model,
            ledger::transaction::account::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + Send
        + Sync
        + 'static,
{
    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        let ledger_xact_type = self.get_journal_entry_type(id).await?;

        let jxact_lines = <R as RepositoryOperations<
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
