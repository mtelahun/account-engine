pub mod external_account {
    use arrayvec::ArrayString;

    use crate::domain::{AccountBookId, AccountId};
    use crate::entity::{entity_type, external_account_type};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub book_id: AccountBookId,
        pub account_no: ArrayString<64>,
        pub entity_type: entity_type::Model,
        pub account_type: external_account_type::Model,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub book_id: AccountBookId,
        pub account_no: ArrayString<64>,
        pub entity_type: entity_type::Model,
        pub account_type: external_account_type::Model,
    }
}
