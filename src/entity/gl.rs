pub mod general_ledger {

    use crate::domain::{
        array_long_string::ArrayLongString, AccountId, ArrayCodeString, GeneralLedgerId,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub name: ArrayLongString,
        pub currency_code: ArrayCodeString,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ActiveModel {
        pub id: GeneralLedgerId,
        pub name: ArrayLongString,
        pub root: AccountId,
        pub currency_code: ArrayCodeString,
    }
}
