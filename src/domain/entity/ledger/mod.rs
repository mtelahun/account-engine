use crate::{
    resource::ledger::{self, LedgerType},
    shared_kernel::{ArrayString24, ArrayString3, ArrayString64},
};

use self::ledger_id::LedgerId;

use super::account_balance::AccountBalance;

pub mod ledger_id;

pub trait LedgerOps {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ledger<S: LedgerOps> {
    pub id: LedgerId,
    pub number: ArrayString24,
    pub ledger_type: LedgerType,
    pub parent_id: Option<LedgerId>,
    pub name: ArrayString64,
    pub currency_code: Option<ArrayString3>,
    extra: S,
}

#[derive(Clone, Copy, Debug)]
pub enum LedgerAccount {
    Derived(Ledger<ledger::derived::ActiveModel>),
    Intermediate(Ledger<ledger::intermediate::ActiveModel>),
    Leaf(Ledger<ledger::leaf::ActiveModel>),
}

impl LedgerOps for ledger::derived::ActiveModel {}

impl LedgerOps for ledger::intermediate::ActiveModel {}

impl LedgerOps for ledger::leaf::ActiveModel {}

impl Ledger<ledger::derived::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}

impl Ledger<ledger::intermediate::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}

impl Ledger<ledger::leaf::ActiveModel> {
    pub fn balance(&self) -> AccountBalance {
        todo!()
    }
}
impl<S: LedgerOps> Ledger<S> {
    pub fn new(l: ledger::ActiveModel, s: S) -> Ledger<S> {
        Self {
            id: l.id,
            number: l.number,
            ledger_type: l.ledger_type,
            parent_id: l.parent_id,
            name: l.name,
            currency_code: l.currency_code,
            extra: s,
        }
    }
}

impl LedgerAccount {
    pub fn id(&self) -> LedgerId {
        match self {
            LedgerAccount::Derived(l) => l.id,
            LedgerAccount::Intermediate(l) => l.id,
            LedgerAccount::Leaf(l) => l.id,
        }
    }

    pub fn name(&self) -> ArrayString64 {
        match self {
            LedgerAccount::Derived(l) => l.name,
            LedgerAccount::Intermediate(l) => l.name,
            LedgerAccount::Leaf(l) => l.name,
        }
    }

    pub fn number(&self) -> ArrayString24 {
        match self {
            LedgerAccount::Derived(l) => l.number,
            LedgerAccount::Intermediate(l) => l.number,
            LedgerAccount::Leaf(l) => l.number,
        }
    }
}
