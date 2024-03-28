use postgres_types::{FromSql, ToSql};

use crate::{
    domain::subsidiary_ledger::account_type::AccountType,
    shared_kernel::array_string_64::ArrayString128,
};

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
