use crate::{
    domain::general_ledger::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId},
    shared_kernel::{array_long_string::ArrayString128, ArrayString3},
};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub name: ArrayString128,
    pub currency_code: ArrayString3,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveModel {
    pub id: GeneralLedgerId,
    pub name: ArrayString128,
    pub root: LedgerId,
    pub currency_code: ArrayString3,
}
