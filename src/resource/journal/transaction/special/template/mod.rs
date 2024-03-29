use crate::{
    domain::special_journal::special_journal_template_id::SpecialJournalTemplateId,
    shared_kernel::ArrayString128,
};

pub mod column;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayString128,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: SpecialJournalTemplateId,
    pub name: ArrayString128,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: SpecialJournalTemplateId::new(),
            name: value.name,
        }
    }
}
