use crate::shared_kernel::{ArrayString128, ArrayString24, ArrayString3};

use super::account_engine::EngineError;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    _code: ArrayString24,
    _legal_name: ArrayString128,
    _default_currency_code: ArrayString3,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    _id: uuid::Uuid,
    _code: ArrayString24,
    _legal_name: ArrayString128,
    _default_currency_code: ArrayString3,
}

impl ActiveModel {
    pub async fn update(&self) -> Result<(), EngineError> {
        todo!()
    }
}
