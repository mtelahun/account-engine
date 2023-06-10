pub mod ledger_leaf {

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

pub mod ledger_derived {

    use arrayvec::ArrayString;

    use crate::domain::{AccountBookId, AccountId};

    #[derive(Clone, Debug)]
    pub struct Model {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
        pub account_book_id: AccountBookId,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub ledger_no: ArrayString<64>,
        pub account_book_id: AccountBookId,
    }
}
