use crate::domain::{ArrayString128, LedgerXactTypeCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: LedgerXactTypeCode,
    pub description: ArrayString128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: LedgerXactTypeCode,
    pub description: ArrayString128,
}
