use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::domain::{
    ids::JournalId, xact_type::XactType, AccountId, ExternalXactTypeCode, JournalTransactionId,
};

use super::{PostingRef, TransactionState};

#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub ledger_id: Option<AccountId>,
    pub account_id: Option<AccountId>,
    pub xact_type: XactType,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub amount: Decimal,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveModel {
    pub journal_id: JournalId,
    pub timestamp: NaiveDateTime,
    pub ledger_id: Option<AccountId>,
    pub account_id: Option<AccountId>,
    pub xact_type: XactType,
    pub xact_type_external: Option<ExternalXactTypeCode>,
    pub amount: Decimal,
    pub state: TransactionState,
    pub posting_ref: Option<PostingRef>,
}

impl ActiveModel {
    pub fn id(&self) -> JournalTransactionId {
        JournalTransactionId::new(self.journal_id, self.timestamp)
    }
}

impl From<ledger::ActiveModel> for ActiveModel {
    fn from(value: ledger::ActiveModel) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            ledger_id: Some(value.ledger_id),
            account_id: None,
            xact_type: value.xact_type,
            xact_type_external: None,
            amount: value.amount,
            state: value.state,
            posting_ref: value.posting_ref,
        }
    }
}

impl From<account::ActiveModel> for ActiveModel {
    fn from(value: account::ActiveModel) -> Self {
        Self {
            journal_id: value.journal_id,
            timestamp: value.timestamp,
            ledger_id: None,
            account_id: Some(value.account_id),
            xact_type: value.xact_type,
            xact_type_external: value.xact_type_external,
            amount: value.amount,
            state: value.state,
            posting_ref: value.posting_ref,
        }
    }
}

pub mod ledger {
    use chrono::NaiveDateTime;
    use postgres_types::{FromSql, ToSql};
    use rust_decimal::Decimal;

    use crate::domain::{ids::JournalId, xact_type::XactType, AccountId, JournalTransactionId};

    use super::{PostingRef, TransactionState};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub ledger_id: AccountId,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub state: TransactionState,
        pub posting_ref: Option<PostingRef>,
    }

    #[derive(Clone, Copy, Debug, ToSql, FromSql, PartialEq, Eq)]
    pub struct ActiveModel {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub ledger_id: AccountId,
        pub xact_type: XactType,
        pub amount: Decimal,
        pub state: TransactionState,
        pub posting_ref: Option<PostingRef>,
    }

    impl ActiveModel {
        pub fn id(&self) -> JournalTransactionId {
            JournalTransactionId::new(self.journal_id, self.timestamp)
        }
    }
}

pub mod account {
    use chrono::NaiveDateTime;
    use postgres_types::{FromSql, ToSql};
    use rust_decimal::Decimal;

    use crate::domain::{
        ids::JournalId, xact_type::XactType, AccountId, ExternalXactTypeCode, JournalTransactionId,
    };

    use super::{PostingRef, TransactionState};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub account_id: AccountId,
        pub xact_type: XactType,
        pub xact_type_external: Option<ExternalXactTypeCode>,
        pub amount: Decimal,
        pub state: TransactionState,
        pub posting_ref: Option<PostingRef>,
    }

    #[derive(Clone, Copy, Debug, ToSql, FromSql)]
    pub struct ActiveModel {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub account_id: AccountId,
        pub xact_type: XactType,
        pub xact_type_external: Option<ExternalXactTypeCode>,
        pub amount: Decimal,
        pub state: TransactionState,
        pub posting_ref: Option<PostingRef>,
    }

    impl ActiveModel {
        pub fn id(&self) -> JournalTransactionId {
            JournalTransactionId::new(self.journal_id, self.timestamp)
        }
    }
}
