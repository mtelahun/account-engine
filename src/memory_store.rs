use rusty_money::iso::Currency;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    accounting::{Account, Ledger, LedgerType},
    storage::{AccountEngineStorage, StorageError},
};

#[derive(Debug, Default)]
pub struct MemoryStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Debug, Default)]
pub struct Inner {
    ledgers: HashMap<String, Ledger>,
    accounts: HashMap<String, Account>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        Self {
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    pub fn account_key(ledger_name: &str, account_number: &str) -> String {
        ledger_name.to_string() + account_number
    }
}

impl Inner {
    pub fn new() -> Inner {
        Self {
            ledgers: HashMap::<String, Ledger>::new(),
            accounts: HashMap::<String, Account>::new(),
        }
    }
}

impl AccountEngineStorage for MemoryStore {
    fn accounts(&self, ledger: &Ledger) -> Vec<Account> {
        let mut res = Vec::<Account>::new();
        let inner = self.inner.read().unwrap();
        for account in inner.accounts.values() {
            if account.ledger.name == ledger.name {
                res.push(account.clone());
            }
        }

        res
    }

    fn ledgers(&self) -> Vec<Ledger> {
        let mut res = Vec::<Ledger>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.ledgers.values() {
            res.push(value.clone())
        }

        res
    }

    fn new_account(
        &self,
        ledger: &Ledger,
        name: &str,
        number: &str,
        ltype: LedgerType,
        currency: Option<&Currency>,
    ) -> Result<Account, StorageError> {
        let account = Account::new(ledger, name, number, ltype, currency.unwrap());
        let mut inner = self.inner.write().unwrap();
        let key = MemoryStore::account_key(&ledger.name, number);
        let search = inner.accounts.get(&key);
        if search.is_none() {
            inner.accounts.insert(key, account.clone());

            return Ok(account);
        }

        Err(StorageError::DuplicateRecord(
            "duplicate account number".into(),
        ))
    }

    fn new_ledger(&self, name: &str, currency: &Currency) -> Result<Box<Ledger>, StorageError> {
        let ledger = Box::<Ledger>::new(Ledger::new(name, currency));
        let mut inner = self.inner.write().unwrap();
        let search = inner.ledgers.get(name);
        if search.is_none() {
            inner.ledgers.insert(name.into(), *ledger.clone());

            return Ok(ledger);
        }

        Err(StorageError::DuplicateRecord(
            "duplicate ledger name".into(),
        ))
    }
}
