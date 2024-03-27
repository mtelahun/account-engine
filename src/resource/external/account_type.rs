use crate::domain::{account_type::AccountType, ArrayString128, XactType};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub xact_type_code: XactType,
    pub code: AccountType,
    pub description: ArrayString128,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub xact_type_code: XactType,
    pub code: AccountType,
    pub description: ArrayString128,
}

impl From<Model> for ActiveModel {
    fn from(value: Model) -> Self {
        Self {
            xact_type_code: value.xact_type_code,
            code: value.code,
            description: value.description,
        }
    }
}
