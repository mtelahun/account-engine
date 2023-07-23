use crate::{
    domain::{
        ids::JournalId, GeneralLedgerId, JournalTransactionId, LedgerId, LedgerXactTypeCode,
        PeriodId, SubLedgerId,
    },
    service::ServiceError,
    store::ResourceOperations,
    Store,
};

use super::{
    accounting_period, external, general_ledger, journal, ledger, ledger_xact_type, organization,
    subsidiary_ledger, LedgerKey,
};

pub struct AccountEngine<R>
where
    R: Store
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
        + ResourceOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            crate::resource::journal::transaction::record::Model,
            crate::resource::journal::transaction::record::ActiveModel,
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
        > + ResourceOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + ResourceOperations<external::account::Model, external::account::ActiveModel, LedgerId>,
{
    pub(crate) repository: R,
}

impl<R> AccountEngine<R>
where
    R: Store
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, LedgerId>
        + ResourceOperations<ledger::transaction::Model, ledger::transaction::ActiveModel, LedgerKey>
        + ResourceOperations<
            ledger::transaction::ledger::Model,
            ledger::transaction::ledger::ActiveModel,
            LedgerKey,
        > + ResourceOperations<journal::Model, journal::ActiveModel, JournalId>
        + ResourceOperations<
            crate::resource::journal::transaction::record::Model,
            crate::resource::journal::transaction::record::ActiveModel,
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
        > + ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + ResourceOperations<subsidiary_ledger::Model, subsidiary_ledger::ActiveModel, SubLedgerId>
        + ResourceOperations<external::account::Model, external::account::ActiveModel, LedgerId>
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
