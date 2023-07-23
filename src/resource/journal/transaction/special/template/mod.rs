use crate::domain::{ArrayLongString, SpecJournalTemplateId};

pub mod column;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayLongString,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: SpecJournalTemplateId,
    pub name: ArrayLongString,
}
