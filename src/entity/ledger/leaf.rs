use crate::domain::AccountId;

#[derive(Clone, Debug)]
pub struct Model {
    pub id: AccountId,
}

#[derive(Clone, Debug)]
pub struct ActiveModel {
    pub id: AccountId,
}
