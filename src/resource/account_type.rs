use postgres_types::{FromSql, ToSql};

use crate::domain::{account_type::AccountType, array_long_string::ArrayString128};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: AccountType,
    pub description: ArrayString128,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, FromSql, ToSql)]
pub struct ActiveModel {
    pub code: AccountType,
    pub description: ArrayString128,
}
