use chrono::{Datelike, NaiveDate};
use chronoutil::RelativeDuration;
use rusty_money::iso::Currency;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

use crate::{
    accounting::{
        journal_transaction::{PostingRef, TransactionState},
        ledger_entry::{
            ExternalTransaction, JournalEntry, LedgerKey, LedgerTransaction, LedgerXactType,
        },
        period::{InterimPeriod, InterimType},
        Account, AccountingPeriod, Journal, JournalTransaction, JournalTransactionModel, Ledger,
        LedgerEntry, LedgerType,
    },
    domain::{ids::JournalTransactionId, xact_type::XactType, AccountId, LedgerXactTypeCode},
    storage::{AccountEngineStorage, StorageError},
};

#[derive(Clone, Debug, Default)]
pub struct MemoryStore {
    inner: Arc<RwLock<Inner>>,
}

#[derive(Clone, Debug, Default)]
pub struct Inner {
    ledgers: HashMap<String, Ledger>,
    accounts: HashMap<AccountId, Account>,
    periods: HashMap<i32, AccountingPeriod>,
    journals: HashMap<String, Journal>,
    journal_txs: HashMap<JournalTransactionId, JournalTransaction>,
    journal_entries: HashMap<LedgerKey, LedgerEntry>,
    ledger_txs: HashMap<LedgerKey, LedgerTransaction>,
    _ext_account_txs: HashMap<LedgerKey, ExternalTransaction>,
    ledger_xact_type: HashMap<LedgerXactTypeCode, LedgerXactType>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        Self {
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    fn get_journal_entry_type(&self, _jxact_id: JournalTransactionId) -> LedgerXactType {
        let inner = self.inner.read().unwrap();

        *inner
            .ledger_xact_type
            .get(&LedgerXactTypeCode::from_str("AL").unwrap())
            .unwrap()
    }
}

impl Inner {
    pub fn new() -> Inner {
        let code = LedgerXactTypeCode::from_str("AL").unwrap();
        let mut res = Self {
            ledgers: HashMap::<String, Ledger>::new(),
            accounts: HashMap::<AccountId, Account>::new(),
            periods: HashMap::<i32, AccountingPeriod>::new(),
            journals: HashMap::<String, Journal>::new(),
            journal_txs: HashMap::<JournalTransactionId, JournalTransaction>::new(),
            journal_entries: HashMap::<LedgerKey, LedgerEntry>::new(),
            ledger_txs: HashMap::<LedgerKey, LedgerTransaction>::new(),
            _ext_account_txs: HashMap::<LedgerKey, ExternalTransaction>::new(),
            ledger_xact_type: HashMap::<LedgerXactTypeCode, LedgerXactType>::new(),
        };
        res.ledger_xact_type.insert(code, LedgerXactType { code });

        res
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

    fn account_by_id(&self, ledger: &Ledger, account_id: AccountId) -> Option<Account> {
        let inner = self.inner.read().unwrap();
        for account in inner.accounts.values() {
            if account.ledger.name == ledger.name && account.id == account_id {
                return Some(account.clone());
            }
        }

        None
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
        for value in inner.accounts.values() {
            if value.number == number && value.ledger.name == ledger.name {
                return Err(StorageError::DuplicateRecord(
                    "duplicate account number".into(),
                ));
            }
        }
        let search = inner.accounts.get(&account.id);
        if search.is_none() {
            inner.accounts.insert(account.id, account.clone());

            return Ok(account);
        }

        Err(StorageError::DuplicateRecord("duplicate account ID".into()))
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
            state: TransactionState::Posted,
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

    fn journal_transaction_by_id(&self, id: JournalTransactionId) -> Option<JournalTransaction> {
        let inner = self.inner.read().unwrap();
        for value in inner.journal_txs.values() {
            if value.id == id {
                return Some(value.clone());
            }
        }

        None
    }

    fn post_journal_transaction(&self, jxact_id: JournalTransactionId) -> bool {
        let ledger_xact_type = self.get_journal_entry_type(jxact_id);

        let mut inner = self.inner.write().unwrap();
        let mut xact = match inner.journal_txs.get_mut(&jxact_id) {
            None => return false,
            Some(value) => value,
        };

        let key = LedgerKey {
            ledger_no: xact.account_cr.id,
            datetime: xact.timestamp,
        };
        xact.posting_ref = Some(PostingRef::new(key, ledger_xact_type));
        xact.state = TransactionState::Posted;

        let entry = LedgerEntry {
            ledger_no: key.ledger_no,
            datetime: xact.timestamp,
            ledger_xact_type,
            amount: xact.amount,
            journal_ref: xact.id,
        };
        let tx_dr = LedgerTransaction {
            ledger_no: key.ledger_no,
            datetime: xact.timestamp,
            ledger_dr: xact.account_dr.id,
        };

        inner.journal_entries.insert(key, entry);
        inner.ledger_txs.insert(key, tx_dr);

        true
    }

    fn ledger_entries_by_account_id(&self, account_id: AccountId) -> Vec<LedgerEntry> {
        let mut res = Vec::<LedgerEntry>::new();
        let inner = self.inner.read().unwrap();
        for (key, entry) in &inner.journal_entries {
            if key.ledger_no == account_id {
                res.append(&mut vec![*entry]);
            }
        }

        res
    }

    fn ledger_transactions_by_account_id(&self, account_id: AccountId) -> Vec<LedgerTransaction> {
        let mut res = Vec::<LedgerTransaction>::new();
        let inner = self.inner.read().unwrap();
        for tx in inner.ledger_txs.values() {
            if tx.ledger_dr == account_id {
                res.push(*tx);
            }
        }

        res
    }

    fn ledger_entry_by_key(&self, key: LedgerKey) -> Option<LedgerEntry> {
        let inner = self.inner.read().unwrap();
        if let Some(entry) = inner.journal_entries.get(&key) {
            return Some(*entry);
        };

        None
    }

    fn ledger_transaction_by_key(&self, key: LedgerKey) -> Option<LedgerTransaction> {
        let inner = self.inner.read().unwrap();
        for tx in inner.ledger_txs.values() {
            if tx.ledger_no == key.ledger_no && tx.datetime == key.datetime {
                return Some(*tx);
            }
        }

        None
    }

    fn ledger_entry_by_ref(&self, posting_ref: PostingRef) -> Option<LedgerEntry> {
        if let Some(res) = self.ledger_entry_by_key(posting_ref.ledger_key()) {
            return Some(res);
        }

        None
    }

    fn journal_entries_by_account_id(&self, account_id: AccountId) -> Vec<JournalEntry> {
        let mut res = Vec::<JournalEntry>::new();
        let entries = self.ledger_entries_by_account_id(account_id);
        let xacts = self.ledger_transactions_by_account_id(account_id);
        for e in entries {
            res.push(JournalEntry {
                ledger_no: e.ledger_no,
                datetime: e.datetime,
                xact_type: XactType::Cr,
                amount: e.amount,
                journal_ref: e.journal_ref,
            })
        }
        for t in xacts {
            let counterpart = self
                .ledger_entry_by_key(LedgerKey {
                    ledger_no: t.ledger_no,
                    datetime: t.datetime,
                })
                .unwrap();
            res.push(JournalEntry {
                ledger_no: t.ledger_dr,
                datetime: t.datetime,
                xact_type: XactType::Dr,
                amount: counterpart.amount,
                journal_ref: counterpart.journal_ref,
            })
        }

        res
    }

    fn journal_entries_by_key(&self, key: LedgerKey) -> Vec<JournalEntry> {
        let mut res = Vec::<JournalEntry>::new();
        let le = self.ledger_entry_by_key(key).unwrap();
        let lt = self.ledger_transaction_by_key(key).unwrap();
        res.push(JournalEntry {
            ledger_no: le.ledger_no,
            datetime: le.datetime,
            xact_type: XactType::Cr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });
        res.push(JournalEntry {
            ledger_no: lt.ledger_dr,
            datetime: lt.datetime,
            xact_type: XactType::Dr,
            amount: le.amount,
            journal_ref: le.journal_ref,
        });

        res
    }

    fn journal_entries_by_ref(&self, posting_ref: PostingRef) -> Vec<JournalEntry> {
        self.journal_entries_by_key(posting_ref.ledger_key())
    }
}
