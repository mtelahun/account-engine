use postgres_types::{FromSql, ToSql};

pub mod account;
pub mod intermediate;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "ledgertype")]
pub enum LedgerType {
    #[postgres(name = "derived")]
    Derived,
    #[postgres(name = "intermediate")]
    Intermediate,
    #[postgres(name = "leaf")]
    Leaf,
}

pub mod ledger {
    use crate::domain::{
        array_long_string::ArrayLongString, array_short_string::ArrayShortString, AccountId,
        ArrayCodeString,
    };

    use super::LedgerType;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_no: ArrayShortString,
        pub ledger_type: LedgerType,
        pub parent_id: Option<AccountId>,
        pub name: ArrayLongString,
        pub currency_code: Option<ArrayCodeString>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub ledger_no: ArrayShortString,
        pub ledger_type: LedgerType,
        pub parent_id: Option<AccountId>,
        pub name: ArrayLongString,
        pub currency_code: Option<ArrayCodeString>,
    }
}

pub mod account_type {
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
}
