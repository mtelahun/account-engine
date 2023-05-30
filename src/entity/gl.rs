pub mod general_ledger {
    use rusty_money::iso;

    use crate::domain::{AccountId, LedgerId};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub name: &'static str,
        pub currency: iso::Currency,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ActiveModel {
        pub id: LedgerId,
        pub name: &'static str,
        pub root: AccountId,
        pub currency: iso::Currency,
    }
}
