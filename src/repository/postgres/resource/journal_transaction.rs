use async_trait::async_trait;

use crate::{
    domain::{AccountId, JournalTransactionId},
    entity::{
        journal_transaction, journal_transaction_line, journal_transaction_line_account,
        journal_transaction_line_ledger, journal_transaction_record, ledger, TransactionState,
    },
    orm::{journal_transaction::JournalTransactionService, OrmError, ResourceOperations},
    repository::postgres::repository::PostgresRepository,
};

#[async_trait]
impl JournalTransactionService for PostgresRepository {
    async fn create(
        &self,
        model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        for line in model.lines.iter() {
            if line.ledger_id.is_none() && line.account_id.is_none() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() && line.account_id.is_some() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields NOT empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() {
                if <Self as ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>>::get(
                    self,
                    Some(&vec![line.ledger_id.unwrap()]),
                )
                .await?
                .is_empty()
                {
                    return Err(OrmError::RecordNotFound(format!(
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

        let jtx = journal_transaction_record::Model {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let record = self.insert(&jtx).await?;

        let mut res_tx_lines = Vec::<journal_transaction_line::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal_transaction_line_ledger::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = self.insert(&jtx_line).await?;
                res_tx_lines.push(journal_transaction_line::ActiveModel {
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
                let jtx_line = journal_transaction_line_account::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = self.insert(&jtx_line).await?;
                res_tx_lines.push(journal_transaction_line::ActiveModel {
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

        Ok(journal_transaction::ActiveModel {
            journal_id: record.journal_id,
            timestamp: record.timestamp,
            explanation: record.explanation,
            lines: res_tx_lines,
        })
    }

    async fn retrieve(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction::ActiveModel>, OrmError> {
        let xacts = <Self as ResourceOperations<
            journal_transaction_record::Model,
            journal_transaction_record::ActiveModel,
            JournalTransactionId,
        >>::get(self, ids)
        .await?;
        let record_lines = <Self as ResourceOperations<
            journal_transaction_line_ledger::Model,
            journal_transaction_line_ledger::ActiveModel,
            JournalTransactionId,
        >>::get(self, ids)
        .await?;

        if !xacts.is_empty() {
            let mut lines = Vec::<journal_transaction_line::ActiveModel>::new();
            for r in record_lines {
                lines.push(journal_transaction_line::ActiveModel {
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

            return Ok(vec![journal_transaction::ActiveModel {
                journal_id: xacts[0].journal_id,
                timestamp: xacts[0].timestamp,
                explanation: xacts[0].explanation,
                lines,
            }]);
        }

        Ok(Vec::<journal_transaction::ActiveModel>::new())
    }
}
