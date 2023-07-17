use async_trait::async_trait;

use crate::{
    domain::JournalTransactionId,
    entity::journal::{self, transaction::TransactionState},
    resource::{OrmError, ResourceOperations},
};

use super::{repository::MemoryRepository, JournalTxAccountLines, JournalTxLines};

#[async_trait]
impl
    ResourceOperations<
        journal::transaction::Model,
        journal::transaction::ActiveModel,
        JournalTransactionId,
    > for MemoryRepository
{
    async fn insert(
        &self,
        model: &journal::transaction::Model,
    ) -> Result<journal::transaction::ActiveModel, OrmError> {
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
                if self
                    .get(Some(&vec![line.ledger_id.unwrap()]))
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
        let mut inner = self.inner.write().await;
        let search = inner.journal_xact.get(&jtx_id);
        if search.is_some() {
            return Err(OrmError::DuplicateRecord(format!(
                "journal transaction exists: {}",
                jtx_id
            )));
        }

        let mut res_tx_ledger_lines = Vec::<journal::transaction::line::ledger::ActiveModel>::new();
        let mut res_tx_account_lines =
            Vec::<journal::transaction::line::account::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal::transaction::line::ledger::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jxl = inner.journal_xact_line.get_mut(&jtx_id);
                if let Some(jxl) = jxl {
                    jxl.list.push(jtx_line)
                } else {
                    let mut new_value = JournalTxLines::new();
                    new_value.list.push(jtx_line);
                    inner.journal_xact_line.insert(jtx_id, new_value);
                }
                res_tx_ledger_lines.push(journal::transaction::line::ledger::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    ledger_id: jtx_line.ledger_id,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            } else {
                let jtx_line = journal::transaction::line::account::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jxl = inner.journal_xact_line_account.get_mut(&jtx_id);
                if let Some(jxl) = jxl {
                    jxl.list.push(jtx_line)
                } else {
                    let mut new_value = JournalTxAccountLines::new();
                    new_value.list.push(jtx_line);
                    inner.journal_xact_line_account.insert(jtx_id, new_value);
                }
                res_tx_account_lines.push(journal::transaction::line::account::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: jtx_line.timestamp,
                    account_id: jtx_line.account_id,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            }
        }
        let jtx = journal::transaction::record::ActiveModel {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let _ = inner.journal_xact.insert(jtx_id, jtx);

        let mut ledger_lines = Vec::<journal::transaction::line::ActiveModel>::new();
        if !res_tx_ledger_lines.is_empty() {
            for line in res_tx_ledger_lines.iter() {
                ledger_lines.push(journal::transaction::line::ActiveModel {
                    journal_id: model.journal_id,
                    timestamp: line.timestamp,
                    ledger_id: Some(line.ledger_id),
                    account_id: None,
                    xact_type: line.xact_type,
                    amount: line.amount,
                    state: line.state,
                    posting_ref: line.posting_ref,
                });
            }
            let _ = inner.journal_xact_line.insert(
                jtx_id,
                JournalTxLines {
                    list: res_tx_ledger_lines,
                },
            );
        }
        if !res_tx_account_lines.is_empty() {
            let _ = inner.journal_xact_line_account.insert(
                jtx_id,
                JournalTxAccountLines {
                    list: res_tx_account_lines,
                },
            );
        }

        Ok(journal::transaction::ActiveModel {
            journal_id: jtx.journal_id,
            timestamp: jtx.timestamp,
            explanation: jtx.explanation,
            lines: ledger_lines,
        })
    }

    async fn get(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        let mut res = Vec::<journal::transaction::ActiveModel>::new();
        let inner = self.inner.read().await;

        if let Some(ids) = ids {
            for value in inner.journal_xact.values() {
                if ids.iter().any(|id| *id == value.id()) {
                    let mut xact_lines = Vec::<journal::transaction::line::ActiveModel>::new();
                    for txl in inner.journal_xact_line.values() {
                        if ids.iter().any(|id| *id == value.id()) {
                            for line in txl.list.iter() {
                                xact_lines.push(journal::transaction::line::ActiveModel {
                                    journal_id: line.journal_id,
                                    timestamp: line.timestamp,
                                    ledger_id: Some(line.ledger_id),
                                    account_id: None,
                                    xact_type: line.xact_type,
                                    amount: line.amount,
                                    state: line.state,
                                    posting_ref: line.posting_ref,
                                })
                            }
                        }
                    }
                    let tx = journal::transaction::ActiveModel {
                        journal_id: value.journal_id,
                        timestamp: value.timestamp,
                        explanation: value.explanation,
                        lines: xact_lines,
                    };
                    res.push(tx);
                }
            }
        } else {
            for value in inner.journal_xact.values() {
                let mut xact_lines = Vec::<journal::transaction::line::ActiveModel>::new();
                for txl in inner.journal_xact_line.values() {
                    for line in txl.list.iter() {
                        xact_lines.push(journal::transaction::line::ActiveModel {
                            journal_id: line.journal_id,
                            timestamp: line.timestamp,
                            ledger_id: Some(line.ledger_id),
                            account_id: None,
                            xact_type: line.xact_type,
                            amount: line.amount,
                            state: line.state,
                            posting_ref: line.posting_ref,
                        })
                    }
                }
                let tx = journal::transaction::ActiveModel {
                    journal_id: value.journal_id,
                    timestamp: value.timestamp,
                    explanation: value.explanation,
                    lines: xact_lines,
                };
                res.push(tx);
            }
        }

        Ok(res)
    }

    async fn search(
        &self,
        _domain: &str,
    ) -> Result<Vec<journal::transaction::ActiveModel>, OrmError> {
        todo!()
    }

    async fn save(&self, _model: &journal::transaction::ActiveModel) -> Result<u64, OrmError> {
        todo!()
    }

    async fn delete(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn archive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }

    async fn unarchive(&self, _id: JournalTransactionId) -> Result<u64, OrmError> {
        todo!()
    }
}
