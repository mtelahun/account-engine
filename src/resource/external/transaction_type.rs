use crate::{
    domain::subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
    shared_kernel::ArrayString128,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: ExternalXactTypeCode,
    pub description: ArrayString128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: ExternalXactTypeCode,
    pub description: ArrayString128,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            code: value.code,
            description: value.description,
        }
    }
}
