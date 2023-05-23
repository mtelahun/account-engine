use std::sync::Arc;

use rust_decimal::Decimal;
use rusty_money::iso::{self, Currency};

use crate::accounting::ledger::Ledger;

use super::{error::AccountError, JournalEntry};

#[derive(Clone, Debug)]
pub struct Account {
    pub name: String,
    pub number: String,
    pub ltype: LedgerType,
    pub currency: Arc<iso::Currency>,
    _dr_balance: Decimal,
    _cr_balance: Decimal,
    pub ledger: Arc<Ledger>,
    pub children: Vec<Account>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedgerType {
    Intermediate,
    Leaf,
}

impl Account {
    pub fn new(
        ledger: &Ledger,
        name: &str,
        number: &str,
        ltype: LedgerType,
        currency: &Currency,
    ) -> Account {
        let currency = Arc::new(*currency);
        let ledger = Arc::new(ledger.clone());
        Self {
            name: name.to_owned(),
            number: number.to_owned(),
            children: Vec::<Account>::new(),
            _dr_balance: Decimal::ZERO,
            _cr_balance: Decimal::ZERO,
            currency,
            ltype,
            ledger,
        }
    }

    pub fn add_child(&mut self, ac: Account) -> Result<(), AccountError> {
        if self.ltype == LedgerType::Intermediate {
            self.children.push(ac);

            return Ok(());
        }

        Err(AccountError::ValidationError(
            "cannot add children to a leaf account".into(),
        ))
    }

    pub fn journal_entries(&self) -> Vec<JournalEntry> {
        Vec::<JournalEntry>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_money::iso;

    #[test]
    fn test_new() {
        let ledger = Ledger::new("My Company", iso::USD);
        let ac = Account::new(
            &ledger,
            "Assets",
            "1000",
            LedgerType::Intermediate,
            iso::USD,
        );
        assert_eq!(ac.name, "Assets", "name field is correct");
        assert_eq!(ac.number, "1000", "number field is correct");
        assert_eq!(ac.ltype, LedgerType::Intermediate, "ledger type is correct");
        assert_eq!(
            ac._dr_balance,
            Decimal::ZERO,
            "initial debit balance is zero"
        );
        assert_eq!(
            ac._cr_balance,
            Decimal::ZERO,
            "initial debit balance is zero"
        );
        assert_eq!(
            ac.journal_entries().len(),
            0,
            "Initially, there are NO journal entries in an account"
        )
    }

    #[test]
    fn test_add_child_to_leaf() {
        // Arrange
        let ledger = Ledger::new("My Company", iso::USD);
        let mut assets = Account::new(
            &ledger,
            "Assets",
            "1000",
            LedgerType::Intermediate,
            iso::USD,
        );
        let mut cash = Account::new(&ledger, "Cash", "1000", LedgerType::Leaf, iso::USD);
        let act_receivable = Account::new(
            &ledger,
            "Accounts Receivable",
            "1000",
            LedgerType::Intermediate,
            iso::USD,
        );

        // Act
        let res_assets = assets.add_child(cash.clone());
        let res_cash = cash.add_child(act_receivable);

        // Assert
        assert!(res_assets.is_ok(), "adding child to Intermediate succeeds");
        assert!(res_cash.is_err(), "adding child to Leaf fails");
        assert_eq!(
            res_cash.err().unwrap(),
            Err::<(), AccountError>(AccountError::ValidationError(
                "cannot add children to a leaf account".into()
            ))
            .err()
            .unwrap()
        );
    }
}
