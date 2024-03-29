use crate::shared_kernel::{ids::JournalTypeId, ArrayString64};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Model {
    pub name: ArrayString64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: JournalTypeId,
    pub name: ArrayString64,
}
