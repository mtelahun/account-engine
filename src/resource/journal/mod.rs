use crate::domain::{ids::JournalId, ArrayLongString, ArrayShortString};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub name: ArrayLongString,
    pub code: ArrayShortString,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: JournalId,
    pub name: ArrayLongString,
    pub code: ArrayShortString,
}

pub mod transaction;
