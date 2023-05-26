use chrono::NaiveDate;
use rusty_money::iso::Currency;

use crate::{
    accounting::{
        journal_transaction::{JournalTransaction, PostingRef},
        ledger_entry::{JournalEntry, LedgerKey, LedgerTransaction},
        period::InterimType,
        Account, AccountingPeriod, Journal, JournalTransactionModel, Ledger, LedgerEntry,
        LedgerType,
    },
    domain::{AccountId, JournalTransactionId},
};

pub trait AccountEngineStorage {
    fn new_ledger(&self, name: &str, currency: &Currency) -> Result<Box<Ledger>, StorageError>;

    fn new_account(
        &self,
        ledger: &Ledger,
        name: &str,
        number: &str,
        ltype: LedgerType,
        currency: Option<&Currency>,
    ) -> Result<Account, StorageError>;

    fn new_period(
        &self,
        start: NaiveDate,
        itype: InterimType,
    ) -> Result<AccountingPeriod, StorageError>;

    fn new_journal<'a>(&self, journal: &'a Journal) -> Result<&'a Journal, StorageError>;

    fn journals(&self) -> Vec<Journal>;

    fn journals_by_ledger(&self, ledger_name: &str) -> Vec<Journal>;

    fn new_journal_transaction(
        &self,
        tx: JournalTransactionModel,
    ) -> Result<JournalTransaction, StorageError>;

    fn journal_transactions(&self) -> Vec<JournalTransaction>;

    fn journal_transaction_by_id(
        &self,
        jxact_id: JournalTransactionId,
    ) -> Option<JournalTransaction>;

    fn journal_transactions_by_ledger(&self, ledger_name: &str) -> Vec<JournalTransaction>;

    fn post_journal_transaction(&self, jxact_id: JournalTransactionId) -> bool;

    fn accounts(&self, ledger: &Ledger) -> Vec<Account>;

    fn account_by_id(&self, ledger: &Ledger, account_id: AccountId) -> Option<Account>;

    fn accounts_by_number(&self, ledger: &Ledger, number: &str) -> Vec<Account>;

    fn ledgers(&self) -> Vec<Ledger>;

    fn ledgers_by_name(&self, name: &str) -> Vec<Ledger>;

    fn ledger_entry_by_key(&self, key: LedgerKey) -> Option<LedgerEntry>;

    fn ledger_entry_by_ref(&self, posting_ref: PostingRef) -> Option<LedgerEntry>;

    fn ledger_entries_by_account_id(&self, account_id: AccountId) -> Vec<LedgerEntry>;

    fn ledger_transactions_by_account_id(&self, account_id: AccountId) -> Vec<LedgerTransaction>;

    fn ledger_transaction_by_key(&self, key: LedgerKey) -> Option<LedgerTransaction>;

    fn journal_entries_by_account_id(&self, account_id: AccountId) -> Vec<JournalEntry>;

    fn journal_entries_by_key(&self, key: LedgerKey) -> Vec<JournalEntry>;

    fn journal_entries_by_ref(&self, posting_ref: PostingRef) -> Vec<JournalEntry>;

    fn periods(&self) -> Result<Vec<AccountingPeriod>, StorageError>;

    fn add_subsidiary(&self, main: &Ledger, subsidiary: Ledger) -> Result<(), StorageError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StorageError {
    DuplicateRecord(String),
    RecordNotFound,
    Unknown(String),
}

impl std::error::Error for StorageError {}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            StorageError::DuplicateRecord(msg) => format!("DuplicateRecord Error: {}", msg),
            StorageError::RecordNotFound => "RecordNotFound Error".into(),
            StorageError::Unknown(msg) => format!("Unknown Error: {}", msg),
        };
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        // Act
        let err = StorageError::DuplicateRecord(
            "uniqueness constraint violated when creating/updating a record".into(),
        );
        // Assert
        assert_eq!(
            err.to_string(),
            "DuplicateRecord Error: uniqueness constraint violated when creating/updating a record",
            "error string has correct format",
        );

        // Act
        let err = StorageError::Unknown("non-storage failure".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Unknown Error: non-storage failure",
            "error string has correct format",
        );

        // Act
        let err = StorageError::RecordNotFound;
        // Assert
        assert_eq!(
            err.to_string(),
            "RecordNotFound Error",
            "error string has correct format",
        );
    }
}
