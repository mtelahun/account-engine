pub mod ledger_leaf {
    use crate::domain::AccountId;

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: AccountId,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: AccountId,
    }
}

pub mod ledger_derived {
    use crate::domain::{AccountBookId, AccountId};

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: AccountId,
        pub account_book_id: AccountBookId,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub account_book_id: AccountBookId,
    }
}
