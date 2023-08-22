use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use journal::LedgerPostingRef;
use rust_decimal::Decimal;

use crate::{
    domain::{
        ids::{InterimPeriodId, JournalId},
        AccountId, ColumnTotalId, ExternalXactTypeCode, GeneralLedgerId, JournalTransactionId,
        LedgerId, LedgerXactTypeCode, PeriodId, SubJournalTemplateColId, SubJournalTemplateId,
        SubLedgerId, XactType, XACT_ACCOUNT, XACT_LEDGER,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period, external, general_ledger,
        journal::{self, AccountPostingRef},
        ledger, ledger_xact_type, subsidiary_ledger, LedgerKey, TransactionState,
    },
    service::{
        GeneralJournalService, GeneralLedgerService, JournalTransactionService, ServiceError,
        SubsidiaryJournalService, SubsidiaryLedgerService,
    },
    store::{memory::store::MemoryStore, ResourceOperations},
    Store,
};

#[async_trait]
pub trait SubsidiaryJournalTransactionService<R>:
    GeneralJournalService<R>
    + SubsidiaryJournalService<R>
    + GeneralLedgerService<R>
    + SubsidiaryLedgerService<R>
    + JournalTransactionService<R>
where
    R: Store
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<ledger::intermediate::Model, ledger::intermediate::ActiveModel, LedgerId>
        + ResourceOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
        + ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            accounting_period::interim_period::Model,
            accounting_period::interim_period::ActiveModel,
            InterimPeriodId,
        > + ResourceOperations<
            journal::transaction::record::Model,
            journal::transaction::record::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::totals::Model,
            journal::transaction::special::totals::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::column::total::Model,
            journal::transaction::special::column::total::ActiveModel,
            ColumnTotalId,
        > + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
        + ResourceOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + ResourceOperations<
            journal::transaction::special::template::Model,
            journal::transaction::special::template::ActiveModel,
            SubJournalTemplateId,
        > + ResourceOperations<
            journal::transaction::special::template::column::Model,
            journal::transaction::special::template::column::ActiveModel,
            SubJournalTemplateColId,
        > + ResourceOperations<
            ledger::transaction::account::Model,
            ledger::transaction::account::ActiveModel,
            LedgerKey,
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + ResourceOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + ResourceOperations<
            external::transaction_type::Model,
            external::transaction_type::ActiveModel,
            ExternalXactTypeCode,
        > + ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn post_subsidiary_ledger(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        let jxact = <R as ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::get(
            SubsidiaryJournalTransactionService::store(self),
            Some(&vec![id]),
        )
        .await?;
        if jxact.is_empty() {
            return Ok(false);
        }
        let mut jxact = jxact[0];

        let account =
            SubsidiaryLedgerService::get_accounts(self, Some(&vec![jxact.account_id])).await?;
        if account.is_empty() {
            return Ok(false);
        }
        let account = account[0];

        let subledger = SubsidiaryLedgerService::get_subsidiary_ledgers(
            self,
            Some(&vec![account.subledger_id]),
        )
        .await?;
        if subledger.is_empty() {
            return Ok(false);
        }
        let subledger = subledger[0];

        let control_account =
            GeneralLedgerService::get_ledgers(self, Some(&vec![subledger.ledger_account_id]))
                .await?;
        if control_account.is_empty() {
            return Ok(false);
        }
        let control_account = control_account[0];

        let journal =
            GeneralLedgerService::get_journals(self, Some(&vec![jxact.journal_id])).await?;
        if journal.is_empty() {
            return Ok(false);
        }

        let jxact_lines = <R as ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        >>::get(
            SubsidiaryJournalTransactionService::store(self),
            Some(&vec![id]),
        )
        .await?;
        let cr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.cr_ledger_id.is_some())
            .collect::<Vec<_>>();
        let dr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.dr_ledger_id.is_some())
            .collect::<Vec<_>>();

        let sum_cr = cr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount);
        let sum_dr = dr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount);
        if sum_dr == Decimal::ZERO || sum_dr != sum_cr {
            return Err(ServiceError::Validation(
                "the Dr and Cr sides of the transaction must be equal".to_string(),
            ));
        }

        let mut jrnl_ledger_xact_type = XactType::Cr;
        let ledger_xact_type_account = LedgerXactTypeCode::from(XACT_ACCOUNT);
        let ledger_xact_type_ledger = LedgerXactTypeCode::from(XACT_LEDGER);

        for col in jxact_lines.iter() {
            if col.dr_ledger_id.is_none() && col.cr_ledger_id.is_none() {
                break;
            }
            let key_ledger_id = col.cr_ledger_id.unwrap_or(col.dr_ledger_id.unwrap());
            let mut is_account_entry = false;
            if col.dr_ledger_id.is_some() && col.dr_ledger_id.unwrap() == control_account.id {
                is_account_entry = true;
            }
            if col.cr_ledger_id.is_some() && col.cr_ledger_id.unwrap() == control_account.id {
                jrnl_ledger_xact_type = XactType::Dr;
                is_account_entry = true;
            }
            let key = LedgerKey {
                ledger_id: key_ledger_id,
                timestamp: col.timestamp,
            };
            let entry = ledger::transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: match is_account_entry {
                    true => ledger_xact_type_account,
                    false => ledger_xact_type_ledger,
                },
                amount: col.amount,
                journal_ref: id,
            };
            let _ = SubsidiaryJournalTransactionService::store(self)
                .insert(&entry)
                .await?;
            if is_account_entry {
                let account_line = ledger::transaction::account::Model {
                    ledger_id: key.ledger_id,
                    timestamp: key.timestamp,
                    account_id: jxact.account_id,
                    xact_type_code: jrnl_ledger_xact_type,
                    xact_type_external_code: jxact.xact_type_external.unwrap(),
                };
                let _ = SubsidiaryJournalTransactionService::store(self)
                    .insert(&account_line)
                    .await?;
                jxact.posting_ref = Some(AccountPostingRef {
                    key,
                    account_id: jxact.account_id,
                });
                jxact.account_posted_state = TransactionState::Posted;
                let _ = SubsidiaryJournalTransactionService::store(self)
                    .save(&jxact)
                    .await?;
            } else {
                let ledger_line = ledger::transaction::ledger::Model {
                    ledger_id: key.ledger_id,
                    timestamp: key.timestamp,
                    ledger_dr_id: col.dr_ledger_id.unwrap(),
                };
                let _ = SubsidiaryJournalTransactionService::store(self)
                    .insert(&ledger_line)
                    .await?;
            }
        }

        Ok(true)
    }

    async fn post_general_ledger(
        &self,
        journal_id: JournalId,
        ids: &Vec<JournalTransactionId>,
    ) -> Result<bool, ServiceError> {
        let journal_transactions =
            <R as ResourceOperations<
                journal::transaction::special::Model,
                journal::transaction::special::ActiveModel,
                JournalTransactionId,
            >>::get(SubsidiaryJournalTransactionService::store(self), Some(ids))
            .await?;
        let journal_transactions: Vec<journal::transaction::special::ActiveModel> =
            journal_transactions
                .into_iter()
                .filter(|tx| tx.journal_id == journal_id)
                .collect();
        let tx_ids = journal_transactions
            .iter()
            .map(|tx| tx.id())
            .collect::<Vec<JournalTransactionId>>();
        let mut transaction_columns = <R as ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        >>::get(
            SubsidiaryJournalTransactionService::store(self),
            Some(&tx_ids),
        )
        .await?;
        let journal =
            <R as ResourceOperations<journal::Model, journal::ActiveModel, JournalId>>::get(
                SubsidiaryJournalTransactionService::store(self),
                Some(&vec![journal_id]),
            )
            .await?;
        if journal.is_empty() {
            return Err(ServiceError::EmptyRecord(format!(
                "journal id: {journal_id}"
            )));
        }
        let journal = journal[0];

        let template_columns = SubsidiaryJournalTransactionService::store(self)
            .get_journal_transaction_template_columns(journal.template_id.unwrap_or_default())
            .await?;
        if template_columns.is_empty() {
            return Err(ServiceError::Unknown(format!(
                "no template columns found for special journal template: {}",
                journal.template_id.unwrap_or_default()
            )));
        }

        let mut totals = HashMap::<usize, Decimal>::new();
        for tplcol in template_columns.iter() {
            totals.insert(tplcol.sequence, Decimal::ZERO);
        }
        for tplcol in template_columns.iter() {
            let subtotal = totals.get(&tplcol.sequence).unwrap();
            for col in transaction_columns.iter() {
                if col.sequence == tplcol.sequence {
                    totals.insert(tplcol.sequence, subtotal + col.amount);
                    break;
                }
            }
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_micros();
        let now = NaiveDateTime::from_timestamp_micros(now.try_into().unwrap()).unwrap();

        let transaction_totals = journal::transaction::special::totals::Model {
            journal_id: journal.id,
            timestamp: now,
        };
        let transaction_totals =
            <R as ResourceOperations<
                journal::transaction::special::totals::Model,
                journal::transaction::special::totals::ActiveModel,
                JournalTransactionId,
            >>::insert(SubsidiaryJournalService::store(self), &transaction_totals)
            .await?;

        for col in transaction_columns.iter_mut() {
            let total = *totals.get(&col.sequence).unwrap();
            let tpl_col: Option<&journal::transaction::special::template::column::ActiveModel> =
                template_columns.iter().find(|c| c.sequence == col.sequence);
            let tpl_col = tpl_col.ok_or({
                ServiceError::Validation(format!(
                    "column sequence '{}' found in transaction, but is not in template",
                    col.sequence
                ))
            })?;
            let mut ref_cr: Option<LedgerPostingRef> = None;
            let mut ref_dr: Option<LedgerPostingRef> = None;
            if tpl_col.cr_ledger_id.is_some() {
                let cr = tpl_col.cr_ledger_id.unwrap();
                ref_cr = Some(LedgerPostingRef {
                    key: LedgerKey {
                        ledger_id: cr,
                        timestamp: col.timestamp,
                    },
                    ledger_id: cr,
                });
                ref_dr = Some(LedgerPostingRef {
                    key: LedgerKey {
                        ledger_id: cr,
                        timestamp: col.timestamp,
                    },
                    ledger_id: tpl_col.dr_ledger_id.unwrap(),
                });
            }
            let column_total = journal::transaction::special::column::total::Model {
                total,
                transaction_id: transaction_totals.id(),
                sequence: col.sequence,
                posting_ref_cr: ref_cr,
                posting_ref_dr: ref_dr,
            };
            let column_total =
                <R as ResourceOperations<
                    journal::transaction::special::column::total::Model,
                    journal::transaction::special::column::total::ActiveModel,
                    ColumnTotalId,
                >>::insert(SubsidiaryJournalService::store(self), &column_total)
                .await?;
            let dr_line = journal::transaction::general::line::Model {
                journal_id,
                timestamp: now,
                ledger_id: tpl_col.dr_ledger_id.unwrap(),
                xact_type: XactType::Dr,
                amount: total,
                ..Default::default()
            };
            let cr_line = journal::transaction::general::line::Model {
                journal_id,
                timestamp: now,
                ledger_id: tpl_col.cr_ledger_id.unwrap(),
                xact_type: XactType::Cr,
                amount: total,
                ..Default::default()
            };
            let gj_tx = journal::transaction::general::Model {
                journal_id: journal.id,
                timestamp: now,
                explanation: "blah".into(),
                lines: vec![cr_line, dr_line],
            };
            let gj_tx = GeneralJournalService::create_general_transaction(self, &gj_tx).await?;
            if !JournalTransactionService::post_transaction(self, gj_tx.id()).await? {
                return Err(ServiceError::Unknown(format!(
                    "failed to post transaction of column '{}'",
                    col.sequence
                )));
            }

            col.column_total_id = Some(column_total.id);
            col.state = TransactionState::Posted;
            let _ = <R as ResourceOperations<
                journal::transaction::special::column::Model,
                journal::transaction::special::column::ActiveModel,
                JournalTransactionId,
            >>::save(SubsidiaryJournalService::store(self), col)
            .await?;
        }

        Ok(true)
    }

    async fn get_column_total(
        &self,
        id: JournalTransactionId,
        sequence: usize,
    ) -> Result<journal::transaction::special::column::total::ActiveModel, ServiceError> {
        let cols = <R as ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        >>::get(SubsidiaryJournalService::store(self), Some(&vec![id]))
        .await?;

        for col in cols {
            if col.sequence == sequence {
                if col.state != TransactionState::Posted {
                    return Err(ServiceError::Validation(format!(
                        "column {} has not been posted yet",
                        col.sequence
                    )));
                }
                let total_id = match col.column_total_id {
                    Some(id) => id,
                    None => {
                        return Err(ServiceError::Validation(format!(
                            "column {} has been posted but doesn't contain a column total",
                            col.sequence
                        )))
                    }
                };
                let col_total = <R as ResourceOperations<
                    journal::transaction::special::column::total::Model,
                    journal::transaction::special::column::total::ActiveModel,
                    ColumnTotalId,
                >>::get(
                    SubsidiaryJournalService::store(self), Some(&vec![total_id])
                )
                .await?;
                if col_total.is_empty() {
                    return Err(ServiceError::EmptyRecord(format!(
                        "column total id: {}",
                        total_id
                    )));
                }
                return Ok(col_total[0]);
            }
        }

        Err(ServiceError::EmptyRecord(format!(
            "column sequence {sequence} does not exist"
        )))
    }
}

// impl SubsidiaryJournalTransactionService<PostgresStore> for AccountEngine<PostgresStore> {}

impl SubsidiaryJournalTransactionService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
