pub mod account;
pub mod intermediate;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum LedgerType {
    Intermediate,
    Leaf,
}

pub mod ledger {
    use rusty_money::iso;

    use crate::domain::{AccountId, LedgerId};

    use super::LedgerType;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub general_ledger_id: LedgerId,
        pub ledger_no: &'static str,
        pub ledger_type: LedgerType,
        pub name: &'static str,
        pub currency: Option<iso::Currency>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub general_ledger_id: LedgerId,
        pub ledger_no: &'static str,
        pub ledger_type: LedgerType,
        pub name: &'static str,
        pub currency: Option<iso::Currency>,
    }
}

pub mod account_type {
    use crate::domain::account_type::AccountType;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Model {
        pub code: AccountType,
        pub description: &'static str,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct ActiveModel {
        pub code: AccountType,
        pub description: &'static str,
    }
}
