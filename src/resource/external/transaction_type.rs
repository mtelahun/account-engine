use crate::domain::{ArrayString128, ExternalXactTypeCode};

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
