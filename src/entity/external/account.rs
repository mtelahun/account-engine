pub mod external_account {
    use crate::domain::AccountId;
    use crate::entity::{entity_type, external_account_type};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub account_no: &'static str,
        pub entity_type: entity_type::Model,
        pub account_type: external_account_type::Model,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub id: AccountId,
        pub account_no: &'static str,
        pub entity_type: entity_type::Model,
        pub account_type: external_account_type::Model,
    }
}
