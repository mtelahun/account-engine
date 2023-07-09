use crate::domain::{ArrayCodeString, ArrayLongString, ArrayShortString};

use super::account_engine::EngineError;

#[derive(Clone, Copy, Debug)]
pub struct Model {
    _code: ArrayShortString,
    _legal_name: ArrayLongString,
    _default_currency_code: ArrayCodeString,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    _id: uuid::Uuid,
    _code: ArrayShortString,
    _legal_name: ArrayLongString,
    _default_currency_code: ArrayCodeString,
}

impl ActiveModel {
    pub async fn update(&self) -> Result<(), EngineError> {
        todo!()
    }
}
