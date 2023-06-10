pub mod account;
pub mod intermediate;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum LedgerType {
    Derived,
    Intermediate,
    Leaf,
}

pub mod ledger {
    use arrayvec::ArrayString;
    use rusty_money::iso;

    use crate::domain::{AccountId, LedgerId};

    use super::LedgerType;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub general_ledger_id: LedgerId,
        pub ledger_no: ArrayString<64>,
        pub ledger_type: LedgerType,
        pub parent_id: Option<AccountId>,
        pub name: ArrayString<256>,
        pub currency: Option<iso::Currency>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub general_ledger_id: LedgerId,
        pub ledger_no: ArrayString<64>,
        pub ledger_type: LedgerType,
        pub parent_id: Option<AccountId>,
        pub name: ArrayString<256>,
        pub currency: Option<iso::Currency>,
    }
}

pub mod account_type {
    use arrayvec::ArrayString;

    use crate::domain::account_type::AccountType;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Model {
        pub code: AccountType,
        pub description: ArrayString<256>,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct ActiveModel {
        pub code: AccountType,
        pub description: ArrayString<256>,
    }
}
