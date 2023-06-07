pub mod general_ledger {
    use arrayvec::ArrayString;
    use rusty_money::iso;

    use crate::domain::{AccountId, LedgerId};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub name: ArrayString<256>,
        pub currency: iso::Currency,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ActiveModel {
        pub id: LedgerId,
        pub name: ArrayString<256>,
        pub root: AccountId,
        pub currency: iso::Currency,
    }
}
