use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::{AccountId, JournalTransactionId, LedgerXactTypeCode};

use super::journal_entry::LedgerKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Model {
    pub ledger_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub ledger_xact_type_code: LedgerXactTypeCode,
    pub amount: Decimal,
    pub journal_ref: JournalTransactionId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub ledger_id: AccountId,
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

    use crate::{domain::AccountId, resource::ledger::journal_entry::LedgerKey};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub ledger_dr_id: AccountId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub ledger_dr_id: AccountId,
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
        domain::{AccountId, ExternalXactTypeCode, XactType},
        resource::ledger::journal_entry::LedgerKey,
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub external_account_id: AccountId,
        pub xact_type: XactType,
        pub external_xact_type: ExternalXactTypeCode,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub external_account_id: AccountId,
        pub transaction_type: XactType,
        pub external_xact_type: ExternalXactTypeCode,
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
