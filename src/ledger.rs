use std::sync::Arc;

use crate::account::{Account, LedgerType};
use rusty_money::iso;

#[derive(Clone, Debug)]
pub struct Ledger {
    pub name: String,
    pub root: Option<Account>,
    pub currency: Arc<iso::Currency>,
}

impl Ledger {
    pub fn new(name: &str, currency: &iso::Currency) -> Ledger {
        let currency = Arc::new(*currency);
        let mut ledger = Self {
            name: name.into(),
            root: None,
            currency,
        };
        ledger.root = Some(Account::new(
            &ledger,
            name,
            "0",
            LedgerType::Intermediate,
            &ledger.currency,
        ));

        ledger
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ledger = Ledger::new("Company A", iso::EUR);
        let root = ledger.root.unwrap();

        assert_eq!(ledger.name, "Company A", "the name field is correct");
        assert_eq!(
            &(*root.currency),
            iso::EUR,
            "the root account contains the same currency as the ledger"
        )
    }
}
