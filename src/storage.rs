use rusty_money::iso::Currency;

use crate::accounting::{Account, Ledger, LedgerType};

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

    fn accounts(&self, ledger: &Ledger) -> Vec<Account>;

    fn ledgers(&self) -> Vec<Ledger>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StorageError {
    DuplicateRecord(String),
}

impl std::error::Error for StorageError {}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let StorageError::DuplicateRecord(msg) = self;
        write!(f, "DuplicateRecord Error: {}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        let err = StorageError::DuplicateRecord(
            "uniqueness constraint violated when creating/updating a record".into(),
        );

        assert_eq!(
            err.to_string(),
            "DuplicateRecord Error: uniqueness constraint violated when creating/updating a record",
            "error string has correct format",
        )
    }
}
