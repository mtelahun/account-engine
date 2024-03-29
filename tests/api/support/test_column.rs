use account_engine::domain::{entity::external_account::account_id::AccountId, LedgerAccount};
use rust_decimal::Decimal;

use super::{memstore::state::TestState, state_interface::StateInterface, CreateLedgerType};

#[derive(Clone, Copy, Debug)]
pub enum TestColumn {
    AccountDr(TestColumnAccountDr),
    AccountCr(TestColumnAccountCr),
    LedgerDrCr(TestColumnLedgerDrCr),
}

#[derive(Clone, Copy, Debug)]
pub struct TestColumnAccountDr {
    _id: AccountId,
    amount: Decimal,
}

#[derive(Clone, Copy, Debug)]
pub struct TestColumnAccountCr {
    _id: AccountId,
    amount: Decimal,
}

#[derive(Clone, Copy, Debug)]
pub struct TestColumnLedgerDrCr {
    _ledger_dr: LedgerAccount,
    _ledger_cr: LedgerAccount,
    amount: Decimal,
}

impl TestColumn {
    pub async fn new_ledger_drcr(
        state: &TestState,
        use_dr: CreateLedgerType,
        use_cr: CreateLedgerType,
        amount: Decimal,
    ) -> Self {
        TestColumn::LedgerDrCr(TestColumnLedgerDrCr::new(state, use_dr, use_cr, amount).await)
    }

    pub fn new_account_dr(id: AccountId, amount: Decimal) -> Self {
        TestColumn::AccountDr(TestColumnAccountDr::new(id, amount))
    }

    pub fn new_account_cr(id: AccountId, amount: Decimal) -> Self {
        TestColumn::AccountCr(TestColumnAccountCr::new(id, amount))
    }

    pub fn amount(&self) -> Decimal {
        match self {
            TestColumn::LedgerDrCr(l) => l.amount,
            TestColumn::AccountDr(a) => a.amount,
            TestColumn::AccountCr(a) => a.amount,
        }
    }
}

impl TestColumnLedgerDrCr {
    pub async fn new(
        state: &TestState,
        use_dr: CreateLedgerType,
        use_cr: CreateLedgerType,
        amount: Decimal,
    ) -> Self {
        let ledger_dr = match use_dr {
            CreateLedgerType::Random => state.create_ledger_leaf().await,
            CreateLedgerType::Ledger(l) => l,
        };
        let ledger_cr = match use_cr {
            CreateLedgerType::Random => state.create_ledger_leaf().await,
            CreateLedgerType::Ledger(l) => l,
        };
        Self {
            _ledger_dr: ledger_dr,
            _ledger_cr: ledger_cr,
            amount,
        }
    }
}

impl TestColumnAccountDr {
    pub fn new(id: AccountId, amount: Decimal) -> Self {
        Self { _id: id, amount }
    }
}

impl TestColumnAccountCr {
    pub fn new(id: AccountId, amount: Decimal) -> Self {
        Self { _id: id, amount }
    }
}
