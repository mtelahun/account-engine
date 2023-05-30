use chrono::NaiveDateTime;

use crate::domain::AccountId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LedgerKey {
    pub ledger_no: AccountId,
    pub datetime: NaiveDateTime,
}

pub mod journal_entry {
    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{xact_type::XactType, AccountId, JournalTransactionId};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Model {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ActiveModel {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }
}

pub mod ledger_entry {
    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{AccountId, JournalTransactionId};

    use super::ledger_xact_type;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub ledger_xact_type: ledger_xact_type::ActiveModel,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub ledger_xact_type: ledger_xact_type::ActiveModel,
        pub amount: Decimal,
        pub journal_ref: JournalTransactionId,
    }
}

pub mod ledger_transaction {
    use chrono::NaiveDateTime;

    use crate::domain::AccountId;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub ledger_dr: AccountId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub ledger_dr: AccountId,
    }
}

pub mod external_transaction {
    use chrono::NaiveDateTime;

    use crate::domain::{xact_type::XactType, AccountId};

    use super::external_xact_type;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Model {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
        pub xact_type: XactType,
        pub external_xact_type: external_xact_type::ActiveModel,
        pub account_no: AccountId,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ActiveModel {
        pub ledger_no: AccountId,
        pub datetime: NaiveDateTime,
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
