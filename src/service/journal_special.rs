use std::str::FromStr;

use async_trait::async_trait;

use crate::{
    domain::{
        ids::JournalId, ledger_xact_type_code, JournalTransactionId, LedgerId, LedgerXactTypeCode,
    },
    resource::{
        account_engine::AccountEngine, external, journal, ledger, ledger_xact_type,
        TransactionState,
    },
    store::{
        memory::store::MemoryStore, postgres::store::PostgresStore, OrmError, ResourceOperations,
    },
    Store,
};

use super::ServiceError;

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
            journal::transaction::special::line::Model,
            journal::transaction::special::line::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + ResourceOperations<external::account::Model, external::account::ActiveModel, LedgerId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn create_journal_transaction(
        &self,
        model: &journal::transaction::general::Model,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        for line in model.lines.iter() {
            if line.ledger_id.is_none() && line.account_id.is_none() {
                return Err(ServiceError::Validation(format!(
                    "both ledger and account fields empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() && line.account_id.is_some() {
                return Err(ServiceError::Validation(format!(
                    "both ledger and account fields NOT empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() {
                if <R as ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>>::get(
                    self.store(),
                    Some(&vec![line.ledger_id.unwrap()]),
                )
                .await?
                .is_empty()
                {
                    return Err(ServiceError::EmptyRecord(format!(
                        "account id: {}",
                        line.ledger_id.unwrap()
                    )));
                }
            } else if line.account_id.is_some()
                && <R as ResourceOperations<
                    external::account::Model,
                    external::account::ActiveModel,
                    LedgerId,
                >>::get(self.store(), Some(&vec![line.account_id.unwrap()]))
                .await?
                .is_empty()
            {
                return Err(ServiceError::EmptyRecord(format!(
                    "account id: {}",
                    line.account_id.unwrap()
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

        let mut res_tx_lines = Vec::<journal::transaction::general::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal::transaction::general::line::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = <R as ResourceOperations<
                    journal::transaction::general::line::Model,
                    journal::transaction::general::line::ActiveModel,
                    JournalTransactionId,
                >>::insert(self.store(), &jtx_line)
                .await?;
                res_tx_lines.push(journal::transaction::general::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: Some(jtx_line.ledger_id),
                    account_id: None,
                    xact_type: jtx_line.xact_type,
                    xact_type_external: None,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            } else {
                let jtx_line = journal::transaction::special::line::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    xact_type_external: line.xact_type_external,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = <R as ResourceOperations<
                    journal::transaction::special::line::Model,
                    journal::transaction::special::line::ActiveModel,
                    JournalTransactionId,
                >>::insert(self.store(), &jtx_line)
                .await?;
                res_tx_lines.push(journal::transaction::special::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: None,
                    account_id: Some(jtx_line.account_id),
                    xact_type: jtx_line.xact_type,
                    xact_type_external: jtx_line.xact_type_external,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            }
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
        let account_lines = <R as ResourceOperations<
            journal::transaction::special::line::Model,
            journal::transaction::special::line::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;

        if !xacts.is_empty() {
            let mut lines = Vec::<journal::transaction::general::ActiveModel>::new();
            for r in ledger_lines {
                lines.push(journal::transaction::general::ActiveModel {
                    journal_id: r.journal_id,
                    timestamp: r.timestamp,
                    ledger_id: Some(r.ledger_id),
                    account_id: None,
                    xact_type: r.xact_type,
                    xact_type_external: None,
                    amount: r.amount,
                    state: r.state,
                    posting_ref: r.posting_ref,
                })
            }
            for r in account_lines {
                lines.push(journal::transaction::special::ActiveModel {
                    journal_id: r.journal_id,
                    timestamp: r.timestamp,
                    ledger_id: None,
                    account_id: Some(r.account_id),
                    xact_type: r.xact_type,
                    xact_type_external: r.xact_type_external,
                    amount: r.amount,
                    state: r.state,
                    posting_ref: r.posting_ref,
                })
            }

            return Ok(vec![journal::transaction::general::ActiveModel {
                journal_id: xacts[0].journal_id,
                timestamp: xacts[0].timestamp,
                explanation: xacts[0].explanation,
                lines,
            }]);
        }

        Ok(Vec::<journal::transaction::general::ActiveModel>::new())
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
