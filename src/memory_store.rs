use chrono::{Datelike, NaiveDate};
use chronoutil::RelativeDuration;
use rusty_money::iso::Currency;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    accounting::{
        period::{InterimPeriod, InterimType},
        Account, AccountingPeriod, Ledger, LedgerType,
    },
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
    periods: HashMap<i32, AccountingPeriod>,
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
            periods: HashMap::<i32, AccountingPeriod>::new(),
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

    fn new_period(
        &self,
        start: NaiveDate,
        itype: InterimType,
    ) -> Result<AccountingPeriod, StorageError> {
        let end = start + RelativeDuration::years(1) + RelativeDuration::days(-1);
        let mut period = AccountingPeriod {
            fiscal_year: end.year(),
            period_start: start,
            period_end: end,
            periods: Vec::<InterimPeriod>::new(),
        };
        let mut interim_periods = match itype {
            InterimType::CalendarMonth => period.create_interim_calendar(),
            InterimType::FourWeek => todo!(),
            InterimType::FourFourFiveWeek => todo!(),
        }
        .map_err(|e| StorageError::Unknown(e.to_string()))?;
        period.periods.append(&mut interim_periods);

        let mut inner = self.inner.write().unwrap();
        let search = inner.periods.get(&period.fiscal_year);
        if search.is_none() {
            inner.periods.insert(period.fiscal_year, period.clone());

            return Ok(period);
        }

        Err(StorageError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }

    fn periods(&self) -> Result<Vec<AccountingPeriod>, StorageError> {
        let mut res = Vec::<AccountingPeriod>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.periods.values() {
            res.push(value.clone())
        }

        Ok(res)
    }
}
