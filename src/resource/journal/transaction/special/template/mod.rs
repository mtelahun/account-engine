use crate::domain::{ArrayLongString, SubJournalTemplateId};

pub mod column;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayLongString,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: SubJournalTemplateId,
    pub name: ArrayLongString,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: SubJournalTemplateId::new(),
            name: value.name,
        }
    }
}
