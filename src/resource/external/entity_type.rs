use crate::{domain::subsidiary_ledger::entity_code::EntityCode, shared_kernel::ArrayString128};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: EntityCode,
    pub description: ArrayString128,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: EntityCode,
    pub description: ArrayString128,
}
