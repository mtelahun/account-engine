use crate::{
    domain::subsidiary_ledger::entity_code::EntityCode,
    shared_kernel::{ids::ExternalEntityId, ArrayString128},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub entity_type_code: EntityCode,
    pub name: ArrayString128,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: ExternalEntityId,
    pub entity_type_code: EntityCode,
    pub name: ArrayString128,
}
