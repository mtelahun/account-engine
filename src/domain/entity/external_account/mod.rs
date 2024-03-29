use chrono::NaiveDate;

use crate::{
    resource::external,
    shared_kernel::{ids::ExternalEntityId, ArrayString24, ArrayString64},
};

use self::account_id::AccountId;

use super::subsidiary_ledger::subleder_id::SubLedgerId;

pub mod account_id;
pub mod account_transaction_id;
pub mod account_type;

#[derive(Clone, Copy, Debug)]
pub struct ExternalAccountBuilder(external::account::Model);

#[derive(Clone, Copy, Debug)]
pub struct ExternalAccount(external::account::ActiveModel);

impl ExternalAccountBuilder {
    pub fn new(
        subledger_id: &SubLedgerId,
        entity_id: &ExternalEntityId,
        account_no: ArrayString24,
        name: ArrayString64,
        date_opened: NaiveDate,
    ) -> ExternalAccountBuilder {
        let model = external::account::Model {
            subledger_id: *subledger_id,
            entity_id: *entity_id,
            account_no,
            name,
            date_opened,
        };

        Self(model)
    }

    pub(crate) fn to_model(self) -> external::account::Model {
        self.0
    }
}

impl ExternalAccount {
    pub fn account_no(&self) -> ArrayString24 {
        self.0.account_no
    }

    pub fn date_opened(&self) -> NaiveDate {
        self.0.date_opened
    }

    pub fn entity_id(&self) -> ExternalEntityId {
        self.0.entity_id
    }

    pub fn id(&self) -> AccountId {
        self.0.id
    }

    pub fn name(&self) -> ArrayString64 {
        self.0.name
    }

    pub fn subledger_id(&self) -> SubLedgerId {
        self.0.subledger_id
    }
}

impl From<external::account::ActiveModel> for ExternalAccount {
    fn from(value: external::account::ActiveModel) -> Self {
        Self(value)
    }
}
