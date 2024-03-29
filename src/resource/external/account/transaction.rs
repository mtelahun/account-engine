use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    domain::subsidiary_ledger::{
        account_id::AccountId, account_transaction_id::AccountTransactionId,
    },
    shared_kernel::XactType,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub external_account_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub xact_type_code: XactType,
    pub amount: Decimal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub external_account_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub xact_type_code: XactType,
    pub amount: Decimal,
}

impl ActiveModel {
    pub fn id(&self) -> AccountTransactionId {
        AccountTransactionId::new(self.external_account_id, self.timestamp)
    }
}

impl From<Model> for ActiveModel {
    fn from(value: Model) -> Self {
        Self {
            external_account_id: value.external_account_id,
            timestamp: value.timestamp,
            xact_type_code: value.xact_type_code,
            amount: value.amount,
        }
    }
}
