use crate::domain::LedgerId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub id: LedgerId,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub id: LedgerId,
}
