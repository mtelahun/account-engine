use crate::{
    domain::{
        ids::JournalId, AccountId, GeneralLedgerId, JournalTransactionId, LedgerXactTypeCode,
        PeriodId,
    },
    store::ResourceOperations,
    OrmError, Repository,
};

use super::{
    accounting_period, general_ledger, journal, ledger, ledger_xact_type, organization, LedgerKey,
};

pub struct AccountEngine<R>
where
    R: Repository
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
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
        >,
{
    // pub (crate) organization: organization::ActiveModel,
    // pub (crate) general_ledger: general_ledger::ActiveModel,
    // pub (crate) journals: Vec<journal::ActiveModel>,
    // subsidiary_ledgers: Vec<SubsidiaryLedger>,
    // external_entities: Vec<ExternalEntity>,
    pub(crate) repository: R,
}

impl<R> AccountEngine<R>
where
    R: Repository
        + ResourceOperations<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
        + ResourceOperations<ledger::Model, ledger::ActiveModel, AccountId>
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
        > + ResourceOperations<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
        + Send
        + Sync
        + 'static,
{
    pub async fn new(r: R) -> Result<Self, EngineError> {
        // let organization = r.organization().await?;
        // let general_ledger: Vec<general_ledger::ActiveModel> = r.get(None)
        //     .await?;
        // let general_ledger = general_ledger[0];
        // let journals: Vec<journal::ActiveModel> = r.get(None).await?;

        Ok(Self {
            // organization,
            // general_ledger,
            // journals,
            repository: r,
        })
    }

    pub async fn organization(&self) -> Result<organization::ActiveModel, EngineError> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineError {
    Repository(OrmError),
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

impl From<OrmError> for EngineError {
    fn from(value: OrmError) -> Self {
        EngineError::Repository(value)
    }
}
