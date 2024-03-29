use crate::{
    domain::entity::subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
    shared_kernel::ArrayString64,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: ExternalXactTypeCode,
    pub description: ArrayString64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: ExternalXactTypeCode,
    pub description: ArrayString64,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            code: value.code,
            description: value.description,
        }
    }
}
