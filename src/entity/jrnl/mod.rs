pub mod transaction;

pub mod journal {
    use crate::domain::ids::JournalId;

    #[derive(Clone, Debug)]
    pub struct Model {
        pub name: String,
        pub code: String,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: JournalId,
        pub name: String,
        pub code: String,
    }
}
