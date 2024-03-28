use crate::{
    domain::{
        general_ledger::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId},
        period::period_id::PeriodId,
        subsidiary_ledger::{account_id::AccountId, subleder_id::SubLedgerId},
        ServiceError,
    },
    infrastructure::data::db_context::repository_operations::RepositoryOperations,
    shared_kernel::{
        composite_ids::JournalTransactionColumnId, ids::JournalId, JournalTransactionId,
        LedgerXactTypeCode,
    },
    Store,
};

use super::{
    accounting_period, external, general_ledger, journal, ledger, ledger_xact_type, organization,
    subsidiary_ledger, LedgerKey,
};

pub struct AccountEngine<R>
where
    R: Store
        + RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<
            ledger::transaction::Model,
            ledger::transaction::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<
            crate::resource::journal::transaction::Model,
            crate::resource::journal::transaction::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            crate::resource::journal::transaction::special::Model,
            crate::resource::journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + RepositoryOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>,
{
    pub(crate) repository: R,
}

impl<R> AccountEngine<R>
where
    R: Store
        + RepositoryOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + RepositoryOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + RepositoryOperations<
            ledger::transaction::Model,
            ledger::transaction::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + RepositoryOperations<journal::Model, journal::ActiveModel, JournalId>
        + RepositoryOperations<
            crate::resource::journal::transaction::Model,
            crate::resource::journal::transaction::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            crate::resource::journal::transaction::special::Model,
            crate::resource::journal::transaction::special::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::general::line::Model,
            journal::transaction::general::line::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            journal::transaction::column::account_dr::Model,
            journal::transaction::column::account_dr::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::column::account_cr::Model,
            journal::transaction::column::account_cr::ActiveModel,
            JournalTransactionColumnId,
        > + RepositoryOperations<
            journal::transaction::special::column::Model,
            journal::transaction::special::column::ActiveModel,
            JournalTransactionId,
        > + RepositoryOperations<
            ledger_xact_type::Model,
            ledger_xact_type::ActiveModel,
            LedgerXactTypeCode,
        > + RepositoryOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + RepositoryOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + RepositoryOperations<external::account::Model, external::account::ActiveModel, AccountId>
        + Send
        + Sync
        + 'static,
{
    pub async fn new(r: R) -> Result<Self, EngineError> {
        Ok(Self { repository: r })
    }

    pub async fn organization(&self) -> Result<organization::ActiveModel, EngineError> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineError {
    Repository(ServiceError),
    Unknown,
}

impl std::error::Error for EngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EngineError::Repository(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            EngineError::Repository(e) => format!("an error occurred while fetching data: {e}"),
            EngineError::Unknown => "an unexpected error occurred".to_string(),
        };

        write!(f, "{msg}")
    }
}

impl From<ServiceError> for EngineError {
    fn from(value: ServiceError) -> Self {
        EngineError::Repository(value)
    }
}
