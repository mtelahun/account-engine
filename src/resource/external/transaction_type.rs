use crate::domain::{entity_code::EntityCode, ArrayLongString, ExternalXactTypeCode};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub code: ExternalXactTypeCode,
    pub entity_type_code: EntityCode,
    pub description: ArrayLongString,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: ExternalXactTypeCode,
    pub entity_type_code: EntityCode,
    pub description: ArrayLongString,
}
