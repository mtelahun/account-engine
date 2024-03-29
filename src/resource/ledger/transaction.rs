use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::{
    domain::entity::ledger::ledger_id::LedgerId,
    shared_kernel::{JournalTransactionId, LedgerXactTypeCode},
};

use super::journal_entry::LedgerKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub ledger_id: LedgerId,
    pub timestamp: NaiveDateTime,
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub ledger_id: LedgerId,
    pub timestamp: NaiveDateTime,
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

impl ActiveModel {
    pub fn id(&self) -> LedgerKey {
        LedgerKey {
            ledger_id: self.ledger_id,
            timestamp: self.timestamp,
        }
    }
}

pub mod ledger {
    use chrono::NaiveDateTime;

    use crate::{
        domain::entity::ledger::ledger_id::LedgerId, resource::ledger::journal_entry::LedgerKey,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: LedgerId,
        pub timestamp: NaiveDateTime,
        pub ledger_dr_id: LedgerId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_id: LedgerId,
        pub timestamp: NaiveDateTime,
        pub ledger_dr_id: LedgerId,
    }

    impl ActiveModel {
        pub fn id(&self) -> LedgerKey {
            LedgerKey {
                ledger_id: self.ledger_id,
                timestamp: self.timestamp,
            }
        }
    }
}

pub mod account {
    use chrono::NaiveDateTime;

    use crate::{
        domain::{
            entity::ledger::ledger_id::LedgerId,
            subsidiary_ledger::{
                account_id::AccountId, external_xact_type_code::ExternalXactTypeCode,
            },
        },
        resource::ledger::journal_entry::LedgerKey,
        shared_kernel::XactType,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: LedgerId,
        pub timestamp: NaiveDateTime,
        pub account_id: AccountId,
        pub xact_type_code: XactType,
        pub xact_type_external_code: ExternalXactTypeCode,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_id: LedgerId,
        pub timestamp: NaiveDateTime,
        pub account_id: AccountId,
        pub xact_type_code: XactType,
        pub xact_type_external_code: ExternalXactTypeCode,
    }

    impl ActiveModel {
        pub fn id(&self) -> LedgerKey {
            LedgerKey {
                ledger_id: self.ledger_id,
                timestamp: self.timestamp,
            }
        }
    }
}
