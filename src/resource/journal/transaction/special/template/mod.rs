use crate::{
    domain::entity::special_journal_template::special_journal_template_id::SpecialJournalTemplateId,
    shared_kernel::ArrayString64,
};

pub mod column;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayString64,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: SpecialJournalTemplateId,
    pub name: ArrayString64,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: SpecialJournalTemplateId::new(),
            name: value.name,
        }
    }
}
