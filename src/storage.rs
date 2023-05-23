use chrono::NaiveDate;
use rusty_money::iso::Currency;

use crate::accounting::{period::InterimType, Account, AccountingPeriod, Ledger, LedgerType};

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

    fn accounts(&self, ledger: &Ledger) -> Vec<Account>;

    fn ledgers(&self) -> Vec<Ledger>;

    fn periods(&self) -> Result<Vec<AccountingPeriod>, StorageError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StorageError {
    DuplicateRecord(String),
    Unknown(String),
}

impl std::error::Error for StorageError {}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            StorageError::DuplicateRecord(msg) => format!("DuplicateRecord Error: {}", msg),
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
    }
}
