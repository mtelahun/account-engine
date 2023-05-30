pub mod account;

pub mod entity_type {
    use crate::domain::entity_code::EntityCode;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Model {
        pub code: EntityCode,
        pub name: &'static str,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct ActiveModel {
        pub code: EntityCode,
        pub name: &'static str,
    }
}

pub mod external_account_type {
    use crate::domain::ExternalAccountTypeCode;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct Model {
        pub code: ExternalAccountTypeCode,
        pub description: &'static str,
    }

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    pub struct ActiveModel {
        pub code: ExternalAccountTypeCode,
        pub description: &'static str,
    }
}
