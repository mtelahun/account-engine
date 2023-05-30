pub mod ledger_account {

    use crate::domain::AccountId;

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: AccountId,
        pub ledger_no: &'static str,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub ledger_no: &'static str,
    }
}
