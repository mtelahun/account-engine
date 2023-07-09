use crate::domain::entity_code::EntityCode;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: EntityCode,
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: EntityCode,
    pub name: &'static str,
}
