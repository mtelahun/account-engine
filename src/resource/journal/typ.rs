use crate::domain::{ids::JournalTypeId, ArrayString128};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Model {
    pub name: ArrayString128,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: JournalTypeId,
    pub name: ArrayString128,
}
