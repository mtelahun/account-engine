use crate::domain::{ids::JournalTypeId, ArrayLongString};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Model {
    pub name: ArrayLongString,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: JournalTypeId,
    pub name: ArrayLongString,
}
