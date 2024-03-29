use std::str::FromStr;

use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::{
    domain::{
        entity::{external_account::account_id::AccountId, ledger::ledger_id::LedgerId},
        journal_transaction::{JournalTransactionColumn, SpecialJournalTransaction},
        subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
        ServiceError,
    },
    infrastructure::persistence::context::{
        error::OrmError, memory::MemoryStore, postgres::PostgresStore,
        repository_operations::RepositoryOperations,
    },
    resource::{
        account_engine::AccountEngine,
        external,
        journal::{self, transaction::JournalTransactionColumnType},
        ledger, ledger_xact_type,
    },
    shared_kernel::{
        journal_transaction_column_id::JournalTransactionColumnId, ledger_xact_type_code,
        ArrayString128, JournalId, JournalTransactionId, LedgerXactTypeCode,
    },
    Store,
};

use super::{
    special_journal_template_id::SpecialJournalTemplateId, template_column_id::TemplateColumnId,
};

#[async_trait]
pub trait SpecialJournalService<R>
where
    R: Store
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
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
            journal::transaction::special::template::Model,
            journal::transaction::special::template::ActiveModel,
            SpecialJournalTemplateId,
        > + RepositoryOperations<
            journal::transaction::special::template::column::Model,
            journal::transaction::special::template::column::ActiveModel,
            TemplateColumnId,
        > + RepositoryOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn get_subsidiary_transactions_by_journal(
        &self,
        id: JournalId,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    > {
        let domain = format!("journal_id = {id}");
        let records = <R as RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::search(self.store(), &domain)
        .await?;
        let ids: Vec<JournalTransactionId> = records.iter().map(|r| r.id()).collect();

        self.get_special_transactions(Some(&ids)).await
    }

    async fn create_journal_template(
        &self,
        model: &journal::transaction::special::template::Model,
    ) -> Result<journal::transaction::special::template::ActiveModel, ServiceError> {
        let template: journal::transaction::special::template::ActiveModel =
            self.store().insert(model).await?;

        Ok(template)
    }

    async fn create_journal_template_columns(
        &self,
        columns: Vec<&journal::transaction::special::template::column::Model>,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, ServiceError>
    {
        let mut result = Vec::<journal::transaction::special::template::column::ActiveModel>::new();
        for col in columns {
            result.push(self.store().insert(col).await?);
        }

        Ok(result)
    }

    async fn create_special_transaction(
        &self,
        journal_id: JournalId,
        timestamp: NaiveDateTime,
        explanation: ArrayString128,
        template_id: SpecialJournalTemplateId,
        xact_type_external_code: ExternalXactTypeCode,
        line_models: &[JournalTransactionColumn],
    ) -> Result<
        (
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        ),
        ServiceError,
    > {
        let base_model = journal::transaction::Model {
            journal_id,
            timestamp,
            explanation,
        };
        let model = journal::transaction::special::Model {
            journal_id,
            timestamp,
            template_id,
            xact_type_external: Some(xact_type_external_code),
        };
        let base_record = <R as RepositoryOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
            JournalTransactionId,
        >>::insert(self.store(), &base_model)
        .await?;
        let record = <R as RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::insert(self.store(), &model)
        .await?;

        let mut columns: Vec<JournalTransactionColumn> = Vec::new();
        for line in line_models.iter() {
            match line {
                JournalTransactionColumn::LedgerDrCr(col) => {
                    let model = journal::transaction::column::ledger_drcr::Model {
                        journal_id: col.journal_id,
                        timestamp: col.timestamp,
                        template_column_id: col.template_column_id,
                        amount: col.amount,
                        ledger_cr_id: col.ledger_cr_id,
                        ledger_dr_id: col.ledger_dr_id,
                    };
                    let active_model = <R as RepositoryOperations<
                        journal::transaction::column::ledger_drcr::Model,
                        journal::transaction::column::ledger_drcr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::insert(self.store(), &model)
                    .await?;
                    columns.push(active_model.into());
                }
                JournalTransactionColumn::AccountDr(col) => {
                    let model = journal::transaction::column::account_dr::Model {
                        journal_id: col.journal_id,
                        timestamp: col.timestamp,
                        template_column_id: col.template_column_id,
                        account_id: col.account_id,
                        amount: col.amount,
                    };
                    let active_model = <R as RepositoryOperations<
                        journal::transaction::column::account_dr::Model,
                        journal::transaction::column::account_dr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::insert(self.store(), &model)
                    .await?;
                    columns.push(active_model.into());
                }
                JournalTransactionColumn::AccountCr(col) => {
                    let model = journal::transaction::column::account_cr::Model {
                        journal_id: col.journal_id,
                        timestamp: col.timestamp,
                        template_column_id: col.template_column_id,
                        account_id: col.account_id,
                        amount: col.amount,
                    };
                    let active_model = <R as RepositoryOperations<
                        journal::transaction::column::account_cr::Model,
                        journal::transaction::column::account_cr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::insert(self.store(), &model)
                    .await?;
                    columns.push(active_model.into());
                }
                JournalTransactionColumn::Text(_) => todo!(),
            }
        }

        Ok((
            SpecialJournalTransaction::new(&base_record, record),
            columns,
        ))
    }

    async fn get_special_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    > {
        let base_records = <R as RepositoryOperations<
            journal::transaction::Model,
            journal::transaction::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;
        let records = <R as RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?;

        let mut result = Vec::<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>::new();
        for record in records {
            let tpl_cols = <R as RepositoryOperations<
                journal::transaction::special::template::column::Model,
                journal::transaction::special::template::column::ActiveModel,
                TemplateColumnId,
            >>::search(
                self.store(),
                &format!("template_id = {}", record.template_id),
            )
            .await?;
            let mut columns = Vec::<JournalTransactionColumn>::new();
            for tpl_col in tpl_cols {
                match tpl_col.column_type {
                    JournalTransactionColumnType::LedgerDrCr => {
                        let special_columns = <R as RepositoryOperations<
                            journal::transaction::column::ledger_drcr::Model,
                            journal::transaction::column::ledger_drcr::ActiveModel,
                            JournalTransactionColumnId,
                        >>::search(
                            self.store(),
                            &format!(
                                "journal_id = {}, timestamp = {}, template_column_id = {}",
                                record.journal_id, record.timestamp, tpl_col.id
                            ),
                        )
                        .await?;
                        for col in special_columns {
                            columns.push(JournalTransactionColumn::LedgerDrCr(col))
                        }
                    }
                    JournalTransactionColumnType::Text => todo!(),
                    JournalTransactionColumnType::AccountDr => {
                        let special_columns = <R as RepositoryOperations<
                            journal::transaction::column::account_dr::Model,
                            journal::transaction::column::account_dr::ActiveModel,
                            JournalTransactionColumnId,
                        >>::search(
                            self.store(),
                            &format!(
                                "journal_id = {}, timestamp = {}, template_column_id = {}",
                                record.journal_id, record.timestamp, tpl_col.id
                            ),
                        )
                        .await?;
                        for col in special_columns {
                            columns.push(JournalTransactionColumn::AccountDr(col))
                        }
                    }
                    JournalTransactionColumnType::AccountCr => {
                        let special_columns = <R as RepositoryOperations<
                            journal::transaction::column::account_cr::Model,
                            journal::transaction::column::account_cr::ActiveModel,
                            JournalTransactionColumnId,
                        >>::search(
                            self.store(),
                            &format!(
                                "journal_id = {}, timestamp = {}, template_column_id = {}",
                                record.journal_id, record.timestamp, tpl_col.id
                            ),
                        )
                        .await?;
                        for col in special_columns {
                            columns.push(JournalTransactionColumn::AccountCr(col))
                        }
                    }
                    _ => continue,
                }
            }
            for base in base_records.iter() {
                if base.journal_id == record.journal_id && base.timestamp == record.timestamp {
                    result.push((SpecialJournalTransaction::new(base, record), columns));
                    break;
                }
            }
        }

        Ok(result)
    }

    async fn get_special_transaction_columns(
        &self,
        id: JournalTransactionId,
    ) -> Result<Vec<JournalTransactionColumn>, ServiceError> {
        let jtx = <R as RepositoryOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), Some(&vec![id]))
        .await?;
        let jtx = jtx[0];
        let t_columns =
            <R as RepositoryOperations<
                journal::transaction::special::template::column::Model,
                journal::transaction::special::template::column::ActiveModel,
                TemplateColumnId,
            >>::search(self.store(), &format!("template_id = {}", jtx.template_id))
            .await?;
        let mut result = Vec::new();
        for t_col in t_columns {
            match t_col.column_type {
                JournalTransactionColumnType::LedgerDrCr => {
                    let columns = <R as RepositoryOperations<
                        journal::transaction::column::ledger_drcr::Model,
                        journal::transaction::column::ledger_drcr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::search(
                        self.store(),
                        &format!(
                            "journal_id = {}, timestamp = {}, template_column_id = {}",
                            jtx.journal_id, jtx.timestamp, t_col.id
                        ),
                    )
                    .await?;
                    for col in columns {
                        result.push(JournalTransactionColumn::LedgerDrCr(col))
                    }
                }
                JournalTransactionColumnType::AccountDr => {
                    let special_columns = <R as RepositoryOperations<
                        journal::transaction::column::account_dr::Model,
                        journal::transaction::column::account_dr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::search(
                        self.store(),
                        &format!(
                            "journal_id = {}, timestamp = {}, template_column_id = {}",
                            jtx.journal_id, jtx.timestamp, t_col.id
                        ),
                    )
                    .await?;
                    for col in special_columns {
                        result.push(JournalTransactionColumn::AccountDr(col))
                    }
                }
                JournalTransactionColumnType::AccountCr => {
                    let special_columns = <R as RepositoryOperations<
                        journal::transaction::column::account_cr::Model,
                        journal::transaction::column::account_cr::ActiveModel,
                        JournalTransactionColumnId,
                    >>::search(
                        self.store(),
                        &format!(
                            "journal_id = {}, timestamp = {}, template_column_id = {}",
                            jtx.journal_id, jtx.timestamp, t_col.id
                        ),
                    )
                    .await?;
                    for col in special_columns {
                        result.push(JournalTransactionColumn::AccountCr(col))
                    }
                }
                JournalTransactionColumnType::Text => {
                    let columns = <R as RepositoryOperations<
                        journal::transaction::column::text::Model,
                        journal::transaction::column::text::ActiveModel,
                        JournalTransactionColumnId,
                    >>::search(
                        self.store(), &format!("template_column_id = {}", t_col.id)
                    )
                    .await?;
                    for col in columns {
                        result.push(JournalTransactionColumn::Text(col))
                    }
                }
                _ => continue,
            }
        }

        Ok(result)
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
impl SpecialJournalService<PostgresStore> for AccountEngine<PostgresStore> {
    fn store(&self) -> &PostgresStore {
        &self.repository
    }
}

#[async_trait]
impl SpecialJournalService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
