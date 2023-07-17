use crate::domain::{account_type::AccountType, array_long_string::ArrayLongString};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: AccountType,
    pub description: ArrayLongString,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: AccountType,
    pub description: ArrayLongString,
}
