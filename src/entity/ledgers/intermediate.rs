pub mod ledger_intermediate {
    use arrayvec::ArrayString;

    use crate::domain::AccountId;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Model {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
    }
}
