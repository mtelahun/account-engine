use crate::entity::entry::{ledger_xact_type, LedgerKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionState {
    Pending,
    Archived,
    Posted,
}

#[derive(Clone, Copy, Debug)]
pub struct PostingRef(LedgerKey, ledger_xact_type::ActiveModel);

impl PostingRef {
    pub fn new(key_dr: LedgerKey, ledger_xact_type: ledger_xact_type::ActiveModel) -> Self {
        Self(key_dr, ledger_xact_type)
    }

    pub fn ledger_key(&self) -> LedgerKey {
        self.0
    }

    pub fn xact_type(&self) -> ledger_xact_type::ActiveModel {
        self.1
    }
}

pub mod journal_line {
    use chrono::NaiveDateTime;
    use rust_decimal::Decimal;

    use crate::domain::{ids::JournalId, AccountId, JournalTransactionId};

    use super::{PostingRef, TransactionState};

    #[derive(Clone, Debug)]
    pub struct Model {
        pub timestamp: NaiveDateTime,
        pub journal_id: JournalId,
        pub account_dr_id: AccountId,
        pub account_cr_id: AccountId,
        pub amount: Decimal,
        pub state: TransactionState,
        pub description: String,
        pub posting_ref: Option<PostingRef>,
    }

    #[derive(Clone, Debug)]
    pub struct ActiveModel {
        pub id: JournalTransactionId,
        pub timestamp: NaiveDateTime,
        pub journal_id: JournalId,
        pub account_dr_id: AccountId,
        pub account_cr_id: AccountId,
        pub amount: Decimal,
        pub state: TransactionState,
        pub description: String,
        pub posting_ref: Option<PostingRef>,
    }
}
