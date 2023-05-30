pub mod transaction;

pub mod journal {
    use crate::domain::{ids::JournalId, LedgerId};

    #[derive(Clone, Debug)]
    pub struct Model {
        pub name: String,
        pub code: String,
        pub ledger_id: LedgerId,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: JournalId,
        pub name: String,
        pub code: String,
        pub ledger_id: LedgerId,
    }
}
