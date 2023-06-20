use chrono::NaiveDateTime;
use postgres_types::{FromSql, ToSql};

use crate::domain::AccountId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ToSql, FromSql)]
pub struct LedgerKey {
    pub ledger_id: AccountId,
    pub timestamp: NaiveDateTime,
}

impl std::fmt::Display for LedgerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "LedgerKey(id: {}, timestamp: {})",
            self.ledger_id, self.timestamp
        );
        write!(f, "{msg}")
    }
}

pub mod journal_entry {
    use std::str::FromStr;

    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{
        xact_type::XactType, AccountId, JournalTransactionId, LedgerXactTypeCode, XACT_LEDGER,
    };

    use super::ledger_line;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Model {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ActiveModel {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }

    impl From<ledger_line::ActiveModel> for ActiveModel {
        fn from(value: ledger_line::ActiveModel) -> Self {
            let ll_code = LedgerXactTypeCode::from_str(XACT_LEDGER).unwrap();
            let xact_type = if value.ledger_xact_type_code == ll_code {
                XactType::Cr
            } else {
                XactType::Dr
            };
            Self {
                xact_type,
                ledger_id: value.ledger_id,
                timestamp: value.timestamp,
                amount: value.amount,
                journal_ref: value.journal_ref,
            }
        }
    }
}

pub mod ledger_line {
    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{AccountId, JournalTransactionId, LedgerXactTypeCode};

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
}

pub mod ledger_transaction {
    use chrono::NaiveDateTime;

    use crate::domain::AccountId;

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
}

pub mod account_transaction {
    use chrono::NaiveDateTime;

    use crate::domain::{xact_type::XactType, AccountId};

    use super::external_xact_type;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub xact_type: XactType,
        pub external_xact_type: external_xact_type::ActiveModel,
        pub account_id: AccountId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_id: AccountId,
        pub timestamp: NaiveDateTime,
        pub xact_type: XactType,
        pub external_xact_type: external_xact_type::ActiveModel,
        pub account_no: AccountId,
    }
}

pub mod ledger_xact_type {
    use crate::domain::LedgerXactTypeCode;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub code: LedgerXactTypeCode,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub code: LedgerXactTypeCode,
    }
}

pub mod external_xact_type {
    use crate::domain::{ExternalXactTypeCode, LedgerXactTypeCode};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_xact_type_code: LedgerXactTypeCode,
        pub code: ExternalXactTypeCode,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_xact_type_code: LedgerXactTypeCode,
        pub code: ExternalXactTypeCode,
    }
}

pub mod ledger_xact_type_description {
    use crate::domain::LedgerXactTypeCode;

    #[derive(Clone, Debug)]
    struct Model {
        pub _code: LedgerXactTypeCode,
        pub _description: String,
    }
}

pub mod external_xact_type_description {
    use crate::domain::{ExternalXactTypeCode, LedgerXactTypeCode};

    #[derive(Clone, Debug)]
    pub struct ExternalXactTypeDescription {
        pub ledger_xact_type_code: LedgerXactTypeCode,
        pub code: ExternalXactTypeCode,
        pub description: String,
    }
}
