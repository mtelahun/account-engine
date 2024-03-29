use crate::{
    domain::entity::{
        general_ledger::general_ledger_id::GeneralLedgerId, ledger::ledger_id::LedgerId,
    },
    shared_kernel::{array_string_64::ArrayString128, ArrayString3},
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
