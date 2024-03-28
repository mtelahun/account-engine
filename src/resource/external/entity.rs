use crate::shared_kernel::{entity_code::EntityCode, ids::ExternalEntityId, ArrayString128};

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
