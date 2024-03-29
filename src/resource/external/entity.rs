use crate::{
    domain::entity::external_entity::entity_code::EntityCode,
    shared_kernel::{ids::ExternalEntityId, ArrayString64},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub entity_type_code: EntityCode,
    pub name: ArrayString64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: ExternalEntityId,
    pub entity_type_code: EntityCode,
    pub name: ArrayString64,
}
