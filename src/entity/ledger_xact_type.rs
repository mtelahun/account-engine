use crate::domain::{ArrayLongString, LedgerXactTypeCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: LedgerXactTypeCode,
    pub description: ArrayLongString,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: LedgerXactTypeCode,
    pub description: ArrayLongString,
}
