use crate::domain::{
    array_long_string::ArrayLongString, ArrayCodeString, GeneralLedgerId, LedgerId,
};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayLongString,
    pub currency_code: ArrayCodeString,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: GeneralLedgerId,
    pub name: ArrayLongString,
    pub root: LedgerId,
    pub currency_code: ArrayCodeString,
}
