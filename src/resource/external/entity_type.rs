use crate::{
    domain::entity::external_entity::entity_code::EntityCode, shared_kernel::ArrayString64,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: EntityCode,
    pub description: ArrayString64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: EntityCode,
    pub description: ArrayString64,
}
