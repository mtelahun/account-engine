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
        entity::{
            external_account::{
                account_id::AccountId, account_transaction_id::AccountTransactionId,
            },
            general_ledger::general_ledger_id::GeneralLedgerId,
            interim_period::interim_period_id::InterimPeriodId,
            ledger::ledger_id::LedgerId,
            period::period_id::PeriodId,
        },
        journal_transaction::JournalTransactionColumn,
        special_journal::{
            column_total_id::ColumnTotalId, special_journal_template_id::SpecialJournalTemplateId,
            template_column_id::TemplateColumnId,
        },
        subsidiary_ledger::{
            external_xact_type_code::ExternalXactTypeCode, subleder_id::SubLedgerId,
        },
        GeneralJournalService, GeneralLedgerService, JournalTransactionService, ServiceError,
        SpecialJournalService, SubsidiaryLedgerService,
    },
    infrastructure::persistence::context::{
        memory::MemoryStore, postgres::PostgresStore, repository_operations::RepositoryOperations,
    },
    resource::{
        account_engine::AccountEngine,
        accounting_period, external, general_ledger,
        journal::{self, transaction::AccountPostingRef},
        ledger, ledger_xact_type, subsidiary_ledger, LedgerKey, SubsidiaryLedgerKey,
        TransactionState,
    },
    shared_kernel::{
        journal_transaction_column_id::JournalTransactionColumnId, JournalId, JournalTransactionId,
        LedgerXactTypeCode, Sequence,
    },
    Store,
};

#[async_trait]
pub trait SpecialJournalTransactionService<R>:
    GeneralJournalService<R>
    + SpecialJournalService<R>
    + GeneralLedgerService<R>
    + SubsidiaryLedgerService<R>
    + JournalTransactionService<R>
where
    R: Store
        + RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<ledger::derived::Model, ledger::derived::ActiveModel, LedgerId>
        + RepositoryOperations<
            ledger::intermediate::Model,
            ledger::intermediate::ActiveModel,
            LedgerId,
        > + RepositoryOperations<ledger::leaf::Model, ledger::leaf::ActiveModel, LedgerId>
        + RepositoryOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<
            accounting_period::interim_period::Model,
            accounting_period::interim_period::ActiveModel,
            InterimPeriodId,
        > + RepositoryOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::column::ledger_drcr::Model,
            journal::transaction::column::ledger_drcr::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::column::text::Model,
            journal::transaction::column::text::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::column::account_dr::Model,
            journal::transaction::column::account_dr::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::column::account_cr::Model,
            journal::transaction::column::account_cr::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::summary::Model,
            journal::transaction::special::summary::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::column::sum::Model,
            journal::transaction::special::column::sum::ActiveModel,
            ColumnTotalId,
        > + RepositoryOperations<
            ledger::transaction::Model,
            ledger::transaction::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            journal::transaction::special::template::Model,
            journal::transaction::special::template::ActiveModel,
            SpecialJournalTemplateId,
        > + RepositoryOperations<
            journal::transaction::special::template::column::Model,
            journal::transaction::special::template::column::ActiveModel,
            TemplateColumnId,
        > + RepositoryOperations<
            ledger::transaction::account::Model,
            ledger::transaction::account::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + RepositoryOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + RepositoryOperations<
            external::account::transaction::Model,
            external::account::transaction::ActiveModel,
            AccountTransactionId,
        > + RepositoryOperations<
            external::transaction_type::Model,
            external::transaction_type::ActiveModel,
            ExternalXactTypeCode,
        > + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn post_to_account(&self, id: JournalTransactionId) -> Result<bool, ServiceError> {
        fn filter_cr(col: &JournalTransactionColumn) -> bool {
            match col {
                JournalTransactionColumn::LedgerDrCr(_) => true,
                JournalTransactionColumn::Text(_) => false,
                JournalTransactionColumn::AccountDr(_) => false,
                JournalTransactionColumn::AccountCr(_) => false,
            }
        }

        fn filter_dr(col: &JournalTransactionColumn) -> bool {
            match col {
                JournalTransactionColumn::LedgerDrCr(_) => true,
                JournalTransactionColumn::Text(_) => false,
                JournalTransactionColumn::AccountDr(_) => false,
                JournalTransactionColumn::AccountCr(_) => false,
            }
        }

        // let jxact = <R as ResourceOperations<
        //     journal::transaction::special::Model,
        //     journal::transaction::special::ActiveModel,
        //     JournalTransactionId,
        // >>::get(
        //     SpecialJournalTransactionService::store(self),
        //     Some(&vec![id]),
        // )
        // .await?;
        // if jxact.is_empty() {
        //     return Ok(false);
        // }
        // let jxact = jxact[0];

        // let journal =
        //     GeneralLedgerService::get_journals(self, Some(&vec![jxact.journal_id])).await?;
        // if journal.is_empty() {
        //     return Ok(false);
        // }
        // let journal = journal[0];

        // let control_ledger = GeneralLedgerService::get_ledgers(
        //     self,
        //     Some(&vec![journal.control_ledger_id.unwrap()]),
        // )
        // .await?;
        // if control_ledger.is_empty() {
        //     return Ok(false);
        // }
        // let control_ledger = match control_ledger[0] {
        //     LedgerAccount::Derived(l) => l,
        //     _ => {
        //         return Err(ServiceError::Validation(
        //             "journal control account is not a derived ledger account".into(),
        //         ))
        //     }
        // };

        // let subledger = SubsidiaryLedgerService::get_subsidiary_ledgers(
        //     self,
        //     Some(&vec![control_ledger.subsidiary_ledger_id()]),
        // )
        // .await?;
        // if subledger.is_empty() {
        //     return Ok(false);
        // }
        // let subledger = subledger[0];

        let jx = SpecialJournalService::get_special_transactions(self, Some(&vec![id])).await?;
        let (_, jx_cols) = &jx[0];
        let cr_xact_lines = jx_cols
            .iter()
            .filter(|am| filter_cr(am))
            .collect::<Vec<_>>();
        let dr_xact_lines = jx_cols
            .iter()
            .filter(|am| filter_dr(am))
            .collect::<Vec<_>>();

        let sum_cr = cr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount());
        let sum_dr = dr_xact_lines
            .iter()
            .fold(Decimal::ZERO, |acc, el| acc + el.amount());
        if sum_dr == Decimal::ZERO || sum_dr != sum_cr {
            return Err(ServiceError::Validation(
                "the Dr and Cr sides of the transaction must be equal and must be non-zero"
                    .to_string(),
            ));
        }

        for col in jx_cols.iter() {
            match col {
                JournalTransactionColumn::AccountDr(inner) => {
                    let atx = external::account::transaction::Model {
                        external_account_id: inner.account_id,
                        timestamp: col.id().timestamp(),
                        xact_type_code: crate::shared_kernel::XactType::Dr,
                        amount: col.amount(),
                    };
                    let atx = <R as RepositoryOperations<
                        external::account::transaction::Model,
                        external::account::transaction::ActiveModel,
                        AccountTransactionId,
                    >>::insert(
                        SpecialJournalTransactionService::store(self), &atx
                    )
                    .await?;
                    let key = SubsidiaryLedgerKey {
                        account_id: atx.external_account_id,
                        timestamp: atx.timestamp,
                    };
                    let mut model = *inner;
                    model.posting_ref = Some(AccountPostingRef { key });
                    let _ = <R as RepositoryOperations<
                        journal::transaction::column::account_dr::Model,
                        journal::transaction::column::account_dr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::save(
                        SpecialJournalTransactionService::store(self), &model
                    )
                    .await?;
                }
                JournalTransactionColumn::AccountCr(inner) => {
                    let atx = external::account::transaction::Model {
                        external_account_id: inner.account_id,
                        timestamp: col.id().timestamp(),
                        xact_type_code: crate::shared_kernel::XactType::Cr,
                        amount: col.amount(),
                    };
                    let atx = <R as RepositoryOperations<
                        external::account::transaction::Model,
                        external::account::transaction::ActiveModel,
                        AccountTransactionId,
                    >>::insert(
                        SpecialJournalTransactionService::store(self), &atx
                    )
                    .await?;
                    let key = SubsidiaryLedgerKey {
                        account_id: atx.external_account_id,
                        timestamp: atx.timestamp,
                    };
                    let mut model = *inner;
                    model.posting_ref = Some(AccountPostingRef { key });
                    let _ = <R as RepositoryOperations<
                        journal::transaction::column::account_cr::Model,
                        journal::transaction::column::account_cr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::save(
                        SpecialJournalTransactionService::store(self), &model
                    )
                    .await?;
                }
                _ => continue,
            }
        }

        // let mut jrnl_ledger_xact_type = XactType::Cr;
        // let ledger_xact_type_account = LedgerXactTypeCode::from(XACT_ACCOUNT);
        // let ledger_xact_type_ledger = LedgerXactTypeCode::from(XACT_LEDGER);

        // for col in jx_cols.iter() {
        //     if col.dr_ledger_id.is_none() && col.cr_ledger_id.is_none() {
        //         break;
        //     }
        //     let key_ledger_id = col.cr_ledger_id.unwrap_or(col.dr_ledger_id.unwrap());
        //     let mut is_account_entry = false;
        //     if col.dr_ledger_id.is_some() && col.dr_ledger_id.unwrap() == control_ledger.id {
        //         is_account_entry = true;
        //     }
        //     if col.cr_ledger_id.is_some() && col.cr_ledger_id.unwrap() == control_ledger.id {
        //         jrnl_ledger_xact_type = XactType::Dr;
        //         is_account_entry = true;
        //     }
        //     let key = LedgerKey {
        //         ledger_id: key_ledger_id,
        //         timestamp: col.timestamp,
        //     };
        //     let entry = ledger::transaction::Model {
        //         ledger_id: key.ledger_id,
        //         timestamp: key.timestamp,
        //         ledger_xact_type_code: match is_account_entry {
        //             true => ledger_xact_type_account,
        //             false => ledger_xact_type_ledger,
        //         },
        //         amount: col.amount,
        //         journal_ref: id,
        //     };
        //     let _ = SpecialJournalTransactionService::store(self)
        //         .insert(&entry)
        //         .await?;
        //     if is_account_entry {
        //         let account_line = ledger::transaction::account::Model {
        //             ledger_id: key.ledger_id,
        //             timestamp: key.timestamp,
        //             account_id: jxact.account_id,
        //             xact_type_code: jrnl_ledger_xact_type,
        //             xact_type_external_code: jxact.xact_type_external.unwrap(),
        //         };
        //         let _ = SpecialJournalTransactionService::store(self)
        //             .insert(&account_line)
        //             .await?;
        //         jxact.posting_ref = Some(AccountPostingRef {
        //             key,
        //             account_id: jxact.account_id,
        //         });
        //         jxact.account_posted_state = TransactionState::Posted;
        //         let _ = SpecialJournalTransactionService::store(self)
        //             .save(&jxact)
        //             .await?;
        //     } else {
        //         let ledger_line = ledger::transaction::ledger::Model {
        //             ledger_id: key.ledger_id,
        //             timestamp: key.timestamp,
        //             ledger_dr_id: col.dr_ledger_id.unwrap(),
        //         };
        //         let _ = SpecialJournalTransactionService::store(self)
        //             .insert(&ledger_line)
        //             .await?;
        //     }
        // }

        Ok(true)
    }

    async fn post_general_ledger(
        &self,
        journal_id: JournalId,
        ids: &Vec<JournalTransactionId>,
    ) -> Result<bool, ServiceError> {
        let journal_transactions =
            <R as RepositoryOperations<
                journal::transaction::special::Model,
                journal::transaction::special::ActiveModel,
                JournalTransactionId,
            >>::get(SpecialJournalTransactionService::store(self), Some(ids))
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
        let mut transaction_columns =
            <R as RepositoryOperations<
                journal::transaction::special::column::Model,
                journal::transaction::special::column::ActiveModel,
                JournalTransactionId,
            >>::get(SpecialJournalTransactionService::store(self), Some(&tx_ids))
            .await?;
        let journal =
            <R as RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>>::get(
                SpecialJournalTransactionService::store(self),
                Some(&vec![journal_id]),
            )
            .await?;
        if journal.is_empty() {
            return Err(ServiceError::EmptyRecord(format!(
                "journal id: {journal_id}"
            )));
        }
        let journal = journal[0];

        let template_columns = SpecialJournalTransactionService::store(self)
            .get_journal_transaction_template_columns(journal.template_id.unwrap_or_default())
            .await?;
        if template_columns.is_empty() {
            return Err(ServiceError::Unknown(format!(
                "no template columns found for special journal template: {}",
                journal.template_id.unwrap_or_default()
            )));
        }

        let mut totals = HashMap::<Sequence, Decimal>::new();
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

        let transaction_totals = journal::transaction::special::summary::Model {
            journal_id: journal.id,
            timestamp: now,
        };
        let transaction_totals =
            <R as RepositoryOperations<
                journal::transaction::special::summary::Model,
                journal::transaction::special::summary::ActiveModel,
                JournalTransactionId,
            >>::insert(SpecialJournalService::store(self), &transaction_totals)
            .await?;

        for col in transaction_columns.iter_mut() {
            let amount = *totals.get(&col.sequence).unwrap();
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
            let column_total = journal::transaction::special::column::sum::Model {
                amount,
                summary_id: transaction_totals.id(),
                sequence: col.sequence,
                posting_ref_cr: ref_cr,
                posting_ref_dr: ref_dr,
            };
            let column_total =
                <R as RepositoryOperations<
                    journal::transaction::special::column::sum::Model,
                    journal::transaction::special::column::sum::ActiveModel,
                    ColumnTotalId,
                >>::insert(SpecialJournalService::store(self), &column_total)
                .await?;
            let dr_line = journal::transaction::general::line::Model {
                journal_id,
                timestamp: now,
                dr_ledger_id: tpl_col.dr_ledger_id.unwrap(),
                cr_ledger_id: tpl_col.cr_ledger_id.unwrap(),
                amount,
                ..Default::default()
            };
            let gj_tx = journal::transaction::general::Model {
                journal_id: journal.id,
                timestamp: now,
                explanation: "blah".into(),
                lines: vec![dr_line],
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
            let _ = <R as RepositoryOperations<
                journal::transaction::special::column::Model,
                journal::transaction::special::column::ActiveModel,
                JournalTransactionId,
            >>::save(SpecialJournalService::store(self), col)
            .await?;
        }

        Ok(true)
    }

    async fn get_column_total(
        &self,
        id: JournalTransactionId,
        sequence: Sequence,
    ) -> Result<journal::transaction::special::column::sum::ActiveModel, ServiceError> {
        let cols = <R as RepositoryOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        >>::get(SpecialJournalService::store(self), Some(&vec![id]))
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
                let col_total = <R as RepositoryOperations<
                    journal::transaction::special::column::sum::Model,
                    journal::transaction::special::column::sum::ActiveModel,
                    ColumnTotalId,
                >>::get(
                    SpecialJournalService::store(self), Some(&vec![total_id])
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

impl SpecialJournalTransactionService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

impl SpecialJournalTransactionService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
