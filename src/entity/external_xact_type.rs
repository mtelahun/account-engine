use crate::domain::{ArrayLongString, ExternalXactTypeCode, LedgerXactTypeCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub code: ExternalXactTypeCode,
    pub description: ArrayLongString,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub code: ExternalXactTypeCode,
    pub description: ArrayLongString,
}
