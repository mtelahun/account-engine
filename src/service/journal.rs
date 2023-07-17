use std::str::FromStr;

use async_trait::async_trait;

use crate::{
    domain::{
        ids::JournalId, ledger_xact_type_code, AccountId, JournalTransactionId, LedgerXactTypeCode,
    },
    resource::{
        account_engine::AccountEngine, journal, ledger, ledger_xact_type, TransactionState,
    },
    store::{
        memory::store::MemoryStore, postgres::store::PostgresStore, OrmError, ResourceOperations,
    },
    Store,
};

use super::ServiceError;

#[async_trait]
pub trait JournalService<R>
where
    R: Store
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
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn create_journal_transaction(
        &self,
        model: &journal::transaction::Model,
    ) -> Result<journal::transaction::ActiveModel, ServiceError> {
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
                if <R as ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>>::get(
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
            } else if line.account_id.is_some() {
                //     if self
                //         .search_account(Some(vec![line.account_id.unwrap()]))
                //         .await?
                //         .is_empty()
                //     {
                //         return Err(OrmError::RecordNotFound(format!(
                //             "account id: {}",
                //             line.ledger_id.unwrap()
                //         )));
                //     }
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

        let mut res_tx_lines = Vec::<journal::transaction::line::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal::transaction::line::ledger::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = <R as ResourceOperations<
                    journal::transaction::line::ledger::Model,
                    journal::transaction::line::ledger::ActiveModel,
                    JournalTransactionId,
                >>::insert(self.store(), &jtx_line)
                .await?;
                res_tx_lines.push(journal::transaction::line::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: Some(jtx_line.ledger_id),
                    account_id: None,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            } else {
                let jtx_line = journal::transaction::line::account::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = <R as ResourceOperations<
                    journal::transaction::line::account::Model,
                    journal::transaction::line::account::ActiveModel,
                    JournalTransactionId,
                >>::insert(self.store(), &jtx_line)
                .await?;
                res_tx_lines.push(journal::transaction::line::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: None,
                    account_id: Some(jtx_line.account_id),
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            }
        }

        Ok(journal::transaction::ActiveModel {
            journal_id: record.journal_id,
            timestamp: record.timestamp,
            explanation: record.explanation,
            lines: res_tx_lines,
        })
    }

    async fn get_journal_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::ActiveModel>, ServiceError> {
        let xacts = <R as ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;
        let record_lines = <R as ResourceOperations<
            journal::transaction::line::ledger::Model,
            journal::transaction::line::ledger::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;

        if !xacts.is_empty() {
            let mut lines = Vec::<journal::transaction::line::ActiveModel>::new();
            for r in record_lines {
                lines.push(journal::transaction::line::ActiveModel {
                    journal_id: r.journal_id,
                    timestamp: r.timestamp,
                    ledger_id: Some(r.ledger_id),
                    account_id: None,
                    xact_type: r.xact_type,
                    amount: r.amount,
                    state: r.state,
                    posting_ref: r.posting_ref,
                })
            }

            return Ok(vec![journal::transaction::ActiveModel {
                journal_id: xacts[0].journal_id,
                timestamp: xacts[0].timestamp,
                explanation: xacts[0].explanation,
                lines,
            }]);
        }

        Ok(Vec::<journal::transaction::ActiveModel>::new())
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
impl JournalService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl JournalService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
