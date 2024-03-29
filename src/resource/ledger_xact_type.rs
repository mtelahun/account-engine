use crate::{
    domain::entity::ledger_xact_type_code::LedgerXactTypeCode, shared_kernel::ArrayString64,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: LedgerXactTypeCode,
    pub description: ArrayString64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: LedgerXactTypeCode,
    pub description: ArrayString64,
}
