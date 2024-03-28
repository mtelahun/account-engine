use crate::shared_kernel::{entity_code::EntityCode, ArrayString128};

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
