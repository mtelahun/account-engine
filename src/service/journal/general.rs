use std::str::FromStr;

use async_trait::async_trait;

use crate::{
    domain::{
        ids::JournalId, ledger_xact_type_code, JournalTransactionId, LedgerId, LedgerXactTypeCode,
    },
    resource::{account_engine::AccountEngine, journal, ledger, ledger_xact_type},
    service::ServiceError,
    store::{
        memory::store::MemoryStore, postgres::store::PostgresStore, OrmError, ResourceOperations,
    },
    Store,
};

#[async_trait]
pub trait GeneralJournalService<R>
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
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn create_general_transaction(
        &self,
        model: &journal::transaction::general::Model,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        for line in model.lines.iter() {
            if <R as ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>>::get(
                self.store(),
                Some(&vec![line.ledger_id]),
            )
            .await?
            .is_empty()
            {
                return Err(ServiceError::EmptyRecord(format!(
                    "account id: {}",
                    line.ledger_id
                )));
            }
        }

        let jtx = journal::transaction::record::Model {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let record = <R as ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        >>::insert(self.store(), &jtx)
        .await?;

        let mut res_tx_lines = Vec::<journal::transaction::general::line::ActiveModel>::new();
        for line in model.lines.iter() {
            let jtx_line = <R as ResourceOperations<
                journal::transaction::general::line::Model,
                journal::transaction::general::line::ActiveModel,
                JournalTransactionId,
            >>::insert(self.store(), line)
            .await?;
            res_tx_lines.push(jtx_line)
        }

        Ok(journal::transaction::general::ActiveModel {
            journal_id: record.journal_id,
            timestamp: record.timestamp,
            explanation: record.explanation,
            lines: res_tx_lines,
        })
    }

    async fn get_journal_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::general::ActiveModel>, ServiceError> {
        let mut res = Vec::<journal::transaction::general::ActiveModel>::new();
        let xacts = <R as ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;
        let ledger_lines = <R as ResourceOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;

        if !xacts.is_empty() {
            for xact in xacts {
                let mut lines = Vec::<journal::transaction::general::line::ActiveModel>::new();
                for r in ledger_lines.iter() {
                    lines.push(*r)
                }

                res.push(journal::transaction::general::ActiveModel {
                    journal_id: xact.journal_id,
                    timestamp: xact.timestamp,
                    explanation: xact.explanation,
                    lines,
                });
            }
        }

        Ok(res)
    }

    async fn get_journal_entry_type(
        &self,
        _jxact_id: JournalTransactionId,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        let ll_code = LedgerXactTypeCode::from_str(ledger_xact_type_code::XACT_LEDGER).unwrap();

        Ok(self.store().get(Some(&vec![ll_code])).await?[0])
    }
}

#[async_trait]
impl GeneralJournalService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl GeneralJournalService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
