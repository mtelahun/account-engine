use postgres_types::{FromSql, ToSql};

use crate::{domain::AccountId, entity::LedgerKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "transactionstate")]
pub enum TransactionState {
    #[postgres(name = "pending")]
    Pending,
    #[postgres(name = "archived")]
    Archived,
    #[postgres(name = "posted")]
    Posted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionAccountType {
    Account,
    Ledger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToSql, FromSql)]
pub struct PostingRef {
    pub(crate) key: LedgerKey,
    pub(crate) account_id: AccountId,
}

impl std::fmt::Display for PostingRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!(
            "PostingRef{{key: {}, account_id: {}}}",
            self.key, self.account_id
        );
        write!(f, "{msg}")
    }
}

pub mod journal_transaction {
    use chrono::NaiveDateTime;

    use crate::domain::{ids::JournalId, ArrayLongString, JournalTransactionId};

    use super::journal_transaction_line;

    #[derive(Clone, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub explanation: ArrayLongString,
        pub lines: Vec<journal_transaction_line::Model>,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub explanation: ArrayLongString,
        pub lines: Vec<journal_transaction_line::ActiveModel>,
    }

    impl ActiveModel {
        pub fn id(&self) -> JournalTransactionId {
            JournalTransactionId::new(self.journal_id, self.timestamp)
        }

        pub fn posted(&self) -> bool {
            todo!()
        }
    }
}

pub mod journal_transaction_line {
    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{ids::JournalId, xact_type::XactType, AccountId, JournalTransactionId};

    use super::{
        journal_transaction_line_account, journal_transaction_line_ledger, PostingRef,
        TransactionState,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub ledger_id: Option<AccountId>,
        pub account_id: Option<AccountId>,
        pub xact_type: XactType,
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
        pub amount: Decimal,
        pub state: TransactionState,
        pub posting_ref: Option<PostingRef>,
    }

    impl ActiveModel {
        pub fn id(&self) -> JournalTransactionId {
            JournalTransactionId::new(self.journal_id, self.timestamp)
        }
    }

    impl From<journal_transaction_line_ledger::ActiveModel> for ActiveModel {
        fn from(value: journal_transaction_line_ledger::ActiveModel) -> Self {
            Self {
                journal_id: value.journal_id,
                timestamp: value.timestamp,
                ledger_id: Some(value.ledger_id),
                account_id: None,
                xact_type: value.xact_type,
                amount: value.amount,
                state: value.state,
                posting_ref: value.posting_ref,
            }
        }
    }

    impl From<journal_transaction_line_account::ActiveModel> for ActiveModel {
        fn from(value: journal_transaction_line_account::ActiveModel) -> Self {
            Self {
                journal_id: value.journal_id,
                timestamp: value.timestamp,
                ledger_id: None,
                account_id: Some(value.account_id),
                xact_type: value.xact_type,
                amount: value.amount,
                state: value.state,
                posting_ref: value.posting_ref,
            }
        }
    }
}

pub mod journal_transaction_record {
    use chrono::NaiveDateTime;
    use postgres_types::{FromSql, ToSql};

    use crate::domain::{ids::JournalId, ArrayLongString, JournalTransactionId};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub explanation: ArrayLongString,
    }

    #[derive(Clone, Copy, Debug, ToSql, FromSql)]
    pub struct ActiveModel {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub explanation: ArrayLongString,
    }

    impl ActiveModel {
        pub fn id(&self) -> JournalTransactionId {
            JournalTransactionId::new(self.journal_id, self.timestamp)
        }

        pub fn posted(&self) -> bool {
            todo!()
        }
    }
}

pub mod journal_transaction_line_ledger {
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

pub mod journal_transaction_line_account {
    use chrono::NaiveDateTime;
    use postgres_types::{FromSql, ToSql};
    use rust_decimal::Decimal;

    use crate::domain::{ids::JournalId, xact_type::XactType, AccountId, JournalTransactionId};

    use super::{PostingRef, TransactionState};

    #[derive(Clone, Copy, Debug)]
    pub struct Model {
        pub journal_id: JournalId,
        pub timestamp: NaiveDateTime,
        pub account_id: AccountId,
        pub xact_type: XactType,
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

// pub mod journal_line {
//     use chrono::NaiveDateTime;
//     use ledger_xact_type_code::LedgerXactTypeCode;
//     use rust_decimal::Decimal;

//     use crate::{domain::{ids::JournalId, AccountId, JournalLineId, ledger_xact_type_code}, entity::{LedgerKey}};

//     use super::TransactionState;

//     #[derive(Clone, Debug)]
//     pub struct Model {
//         pub timestamp: NaiveDateTime,
//         pub ledger_id: AccountId,
//         pub ledger_xact_type_code: LedgerXactTypeCode,
//         pub journal_id: JournalId,
//         pub amount: Decimal,
//         pub state: TransactionState,
//         pub description: String,
//         pub posting_ref: Option<LedgerKey>,
//     }

//     #[derive(Clone, Debug)]
//     pub struct ActiveModel {
//         pub id: JournalLineId,
//         pub timestamp: NaiveDateTime,
//         pub ledger_id: AccountId,
//         pub ledger_xact_type_code: LedgerXactTypeCode,
//         pub journal_id: JournalId,
//         pub amount: Decimal,
//         pub state: TransactionState,
//         pub description: String,
//         pub posting_ref: Option<LedgerKey>,
//     }
// }

// pub mod journal_line_ledger {
//     use crate::domain::{JournalLineId, AccountId};

//     #[derive(Clone, Debug)]
//     pub struct Model {
//         id: JournalLineId,
//         ledger_dr_id: AccountId,
//     }

//     #[derive(Clone, Debug)]
//     pub struct ActiveModel {
//         id: JournalLineId,
//         ledger_dr_id: AccountId,
//     }
// }

// pub mod journal_line_account {
//     use crate::domain::{JournalLineId, AccountId, xact_type::XactType, ExternalXactTypeCode};

//     #[derive(Clone, Debug)]
//     pub struct Model {
//         id: JournalLineId,
//         account_id: AccountId,
//         transaction_type_code: XactType,
//         transaction_type_external_code: ExternalXactTypeCode,
//     }

//     #[derive(Clone, Debug)]
//     pub struct ActiveModel {
//         id: JournalLineId,
//         account_id: AccountId,
//         transaction_type_code: XactType,
//         transaction_type_external_code: ExternalXactTypeCode,
//     }
// }
