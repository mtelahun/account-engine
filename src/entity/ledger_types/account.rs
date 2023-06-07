pub mod ledger_account {

    use arrayvec::ArrayString;

    use crate::domain::AccountId;

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
    }
}
