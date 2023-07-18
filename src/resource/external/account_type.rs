use crate::domain::{entity_code::EntityCode, ArrayLongString, ExternalAccountTypeCode};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub entity_code: EntityCode,
    pub code: ExternalAccountTypeCode,
    pub description: ArrayLongString,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub entity_code: EntityCode,
    pub code: ExternalAccountTypeCode,
    pub description: ArrayLongString,
}
