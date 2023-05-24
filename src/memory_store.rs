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
        Account, AccountingPeriod, Journal, JournalTransaction, JournalTransactionModel, Ledger,
        LedgerType,
    },
    domain::ids::JournalTransactionId,
    storage::{AccountEngineStorage, StorageError},
};

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Clone, Debug, Default)]
pub struct Inner {
    ledgers: HashMap<String, Ledger>,
    accounts: HashMap<String, Account>,
    periods: HashMap<i32, AccountingPeriod>,
    journals: HashMap<String, Journal>,
    journal_txs: HashMap<JournalTransactionId, JournalTransaction>,
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
            journals: HashMap::<String, Journal>::new(),
            journal_txs: HashMap::<JournalTransactionId, JournalTransaction>::new(),
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

    fn accounts_by_number(&self, ledger: &Ledger, number: &str) -> Vec<Account> {
        let mut res = Vec::<Account>::new();
        let inner = self.inner.read().unwrap();
        for account in inner.accounts.values() {
            if account.ledger.name == ledger.name && account.number == number {
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

    fn ledgers_by_name(&self, name: &str) -> Vec<Ledger> {
        let mut res = Vec::<Ledger>::new();
        let inner = self.inner.read().unwrap();
        let ledger = inner.ledgers.get(name);

        if let Some(ledger) = ledger {
            res.insert(0, ledger.clone());
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

    fn add_subsidiary(&self, main: &Ledger, subsidiary: Ledger) -> Result<(), StorageError> {
        let mut inner = self.inner.write().unwrap();
        let ledger = inner
            .ledgers
            .get_mut(&main.name)
            .ok_or(StorageError::RecordNotFound)?;
        ledger.subsidiaries.push(subsidiary);

        Ok(())
    }

    fn new_journal<'a>(&self, journal: &'a Journal) -> Result<&'a Journal, StorageError> {
        let name = journal.ledger.name.clone() + journal.code.as_str();
        let mut inner = self.inner.write().unwrap();
        let search = inner.journals.get(&name);
        if search.is_none() {
            inner.journals.insert(name, journal.clone());

            return Ok(journal);
        }

        Err(StorageError::DuplicateRecord(
            "duplicate journal code".into(),
        ))
    }

    fn journals(&self) -> Vec<crate::accounting::Journal> {
        let mut res = Vec::<Journal>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.journals.values() {
            res.push(value.clone())
        }

        res
    }

    fn journals_by_ledger(&self, ledger_name: &str) -> Vec<Journal> {
        let mut res = Vec::<Journal>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.journals.values() {
            if value.ledger.name == ledger_name {
                res.insert(0, value.clone());
            }
        }

        res
    }

    fn new_journal_transaction(
        &self,
        tx: JournalTransactionModel,
    ) -> Result<JournalTransaction, StorageError> {
        let id = JournalTransactionId::new();
        let ledger = tx.journal.ledger.clone();
        let account_dr = self.accounts_by_number(&ledger, tx.acc_no_dr.as_str());
        let account_cr = self.accounts_by_number(&ledger, tx.acc_no_cr.as_str());
        if account_dr.is_empty() || account_cr.is_empty() {
            return Err(StorageError::RecordNotFound);
        }
        let tx = JournalTransaction {
            id,
            timestamp: tx.timestamp,
            posted: tx.posted,
            amount: tx.amount,
            description: tx.description,
            posting_ref: tx.posting_ref,
            journal: tx.journal,
            account_dr: Arc::new(account_dr[0].to_owned()),
            account_cr: Arc::new(account_cr[0].to_owned()),
        };
        let mut inner = self.inner.write().unwrap();
        let search = inner.journal_txs.get(&id);
        if search.is_none() {
            inner.journal_txs.insert(id, tx.clone());

            return Ok(tx);
        }

        Err(StorageError::DuplicateRecord(
            "duplicate journal code".into(),
        ))
    }

    fn journal_transactions(&self) -> Vec<JournalTransaction> {
        let mut res = Vec::<JournalTransaction>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.journal_txs.values() {
            res.insert(0, value.clone());
        }

        res
    }

    fn journal_transactions_by_ledger(&self, ledger_name: &str) -> Vec<JournalTransaction> {
        let mut res = Vec::<JournalTransaction>::new();
        let inner = self.inner.read().unwrap();
        for value in inner.journal_txs.values() {
            if value.journal.ledger.name == ledger_name {
                res.insert(0, value.clone());
            }
        }

        res
    }
}
