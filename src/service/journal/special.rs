use std::str::FromStr;

use async_trait::async_trait;

use crate::{
    domain::{
        ids::JournalId, ledger_xact_type_code, AccountId, JournalTransactionId, LedgerId,
        LedgerXactTypeCode, SubJournalTemplateColId, SubJournalTemplateId,
    },
    resource::{account_engine::AccountEngine, external, journal, ledger, ledger_xact_type},
    service::ServiceError,
    store::{memory::store::MemoryStore, OrmError, ResourceOperations},
    Store,
};

#[async_trait]
pub trait SubsidiaryJournalService<R>
where
    R: Store
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        > + ResourceOperations<
            journal::transaction::special::template::Model,
            journal::transaction::special::template::ActiveModel,
            SubJournalTemplateId,
        > + ResourceOperations<
            journal::transaction::special::template::column::Model,
            journal::transaction::special::template::column::ActiveModel,
            SubJournalTemplateColId,
        > + ResourceOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + ResourceOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    fn store(&self) -> &R;

    async fn get_subsidiary_transactions_by_journal(
        &self,
        id: JournalId,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, ServiceError> {
        let domain = format!("journal_id = {id}");
        let txs = <R as ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::search(self.store(), &domain)
        .await?;

        Ok(txs)
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

    async fn create_subsidiary_transaction<'a>(
        &self,
        model: &'a journal::transaction::special::Model,
        line_models: &'a [journal::transaction::special::column::Model],
    ) -> Result<journal::transaction::special::ActiveModel, ServiceError> {
        if <R as ResourceOperations<
            external::account::Model,
            external::account::ActiveModel,
            AccountId,
        >>::get(self.store(), Some(&vec![model.account_id]))
        .await?
        .is_empty()
        {
            return Err(ServiceError::EmptyRecord(format!(
                "account id: {}",
                model.account_id
            )));
        }

        let record = <R as ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::insert(self.store(), model)
        .await?;

        let mut res_tx_lines = Vec::<journal::transaction::special::column::ActiveModel>::new();
        for line in line_models.iter() {
            let jtx_line = <R as ResourceOperations<
                journal::transaction::special::column::Model,
                journal::transaction::special::column::ActiveModel,
                JournalTransactionId,
            >>::insert(self.store(), line)
            .await?;
            res_tx_lines.push(jtx_line);
        }

        Ok(record)
    }

    async fn get_subsidiary_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::ActiveModel>, ServiceError> {
        Ok(<R as ResourceOperations<
            journal::transaction::special::Model,
            journal::transaction::special::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?)
    }

    async fn get_subsidiary_transaction_columns(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, ServiceError> {
        Ok(<R as ResourceOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        >>::get(self.store(), ids)
        .await?)
    }

    async fn get_journal_entry_type(
        &self,
        _jxact_id: JournalTransactionId,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        let ll_code = LedgerXactTypeCode::from_str(ledger_xact_type_code::XACT_LEDGER).unwrap();

        Ok(self.store().get(Some(&vec![ll_code])).await?[0])
    }
}

// #[async_trait]
// impl SubsidiaryJournalService<PostgresStore> for AccountEngine<PostgresStore> {
//     fn store(&self) -> &PostgresStore {
//         &self.repository
//     }
// }

#[async_trait]
impl SubsidiaryJournalService<MemoryStore> for AccountEngine<MemoryStore> {
    fn store(&self) -> &MemoryStore {
        &self.repository
    }
}
