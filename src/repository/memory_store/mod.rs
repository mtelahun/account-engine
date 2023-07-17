pub mod general_ledger;
pub mod interim_period;
pub mod journal;
pub mod journal_transaction_line_account;
pub mod journal_transaction_line_ledger;
pub mod journal_transaction_record;
pub mod ledger;
pub mod ledger_intermediate;
pub mod ledger_leaf;
pub mod ledger_transaction;
pub mod ledger_transaction_ledger;
pub mod ledger_xact_type;
pub mod period;
pub mod repository;

// impl MemoryStore {
//     pub fn new() -> MemoryStore {
//         Self {
//             inner: Arc::new(RwLock::new(Inner::new())),
//         }
//     }

//     pub async fn journal_entries_by_account_id(
//         &self,
//         account_id: AccountId,
//     ) -> Vec<journal_entry::ActiveModel> {
//         let mut res = Vec::<journal_entry::ActiveModel>::new();
//         let entries = self.ledger_entries_by_account_id(account_id).await;
//         let xacts = self.ledger_transactions_by_account_id(account_id).await;
//         for e in entries {
//             res.push(journal_entry::ActiveModel {
//                 ledger_id: e.ledger_id,
//                 timestamp: e.timestamp,
//                 xact_type: XactType::Cr,
//                 amount: e.amount,
//                 journal_ref: e.journal_ref,
//             })
//         }
//         for t in xacts {
//             let counterpart = self
//                 .ledger_entry_by_key(LedgerKey {
//                     ledger_id: t.ledger_id,
//                     timestamp: t.timestamp,
//                 })
//                 .await
//                 .unwrap();
//             res.push(journal_entry::ActiveModel {
//                 ledger_id: t.ledger_dr_id,
//                 timestamp: t.timestamp,
//                 xact_type: XactType::Dr,
//                 amount: counterpart.amount,
//                 journal_ref: counterpart.journal_ref,
//             })
//         }

//         res
//     }

//     pub async fn ledger_entries_by_account_id(
//         &self,
//         account_id: AccountId,
//     ) -> Vec<ledger_line::ActiveModel> {
//         let mut res = Vec::<ledger_line::ActiveModel>::new();
//         let inner = self.inner.read().await;
//         for (key, entry) in &inner.journal_entry {
//             if key.ledger_id == account_id {
//                 res.append(&mut vec![*entry]);
//             }
//         }

//         res
//     }

//     pub async fn ledger_transactions_by_account_id(
//         &self,
//         account_id: AccountId,
//     ) -> Vec<transaction::ledger::ActiveModel> {
//         let mut res = Vec::<transaction::ledger::ActiveModel>::new();
//         let inner = self.inner.read().await;
//         for tx in inner.ledger_xact.values() {
//             if tx.ledger_dr_id == account_id {
//                 res.push(*tx);
//             }
//         }

//         res
//     }

//     pub async fn ledger_entry_by_key(&self, key: LedgerKey) -> Option<ledger_line::ActiveModel> {
//         let inner = self.inner.read().await;
//         if let Some(entry) = inner.journal_entry.get(&key) {
//             return Some(*entry);
//         };

//         None
//     }

//     pub async fn journal_entry_by_ref(
//         &self,
//         posting_ref: PostingRef,
//     ) -> Option<journal_entry::ActiveModel> {
//         let entries = self.journal_entries_by_key(posting_ref.key).await;
//         if !entries.is_empty() {
//             for entry in entries.iter() {
//                 if entry.ledger_id == posting_ref.account_id {
//                     return Some(*entry);
//                 }
//             }
//         }

//         None
//     }

//     async fn journal_entries_by_key(&self, key: LedgerKey) -> Vec<journal_entry::ActiveModel> {
//         let mut res = Vec::<journal_entry::ActiveModel>::new();
//         let le = self.ledger_entry_by_key(key).await.unwrap();
//         let lt = self.ledger_transaction_by_key(key).await.unwrap();
//         res.push(journal_entry::ActiveModel {
//             ledger_id: le.ledger_id,
//             timestamp: le.timestamp,
//             xact_type: XactType::Cr,
//             amount: le.amount,
//             journal_ref: le.journal_ref,
//         });
//         res.push(journal_entry::ActiveModel {
//             ledger_id: lt.ledger_dr_id,
//             timestamp: lt.timestamp,
//             xact_type: XactType::Dr,
//             amount: le.amount,
//             journal_ref: le.journal_ref,
//         });

//         res
//     }

//     async fn ledger_transaction_by_key(
//         &self,
//         key: LedgerKey,
//     ) -> Option<transaction::ledger::ActiveModel> {
//         let inner = self.inner.read().await;
//         for tx in inner.ledger_xact.values() {
//             if tx.ledger_id == key.ledger_id && tx.timestamp == key.timestamp {
//                 return Some(*tx);
//             }
//         }

//         None
//     }

//     // fn ledger_entry_by_ref(&self, posting_ref: PostingRef) -> Option<LedgerEntry::ActiveModel> {
//     //     if let Some(res) = self.ledger_entry_by_key(posting_ref.ledger_key()) {
//     //         return Some(res);
//     //     }

//     //     None
//     // }

//     pub async fn post_journal_transaction(&self, jxact_id: JournalTransactionId) -> bool {
//         let ledger_xact_type = self.get_journal_entry_type(jxact_id).await;

//         let mut inner = self.inner.write().await;
//         let xact_lines = match inner.journal_xact_line.get_mut(&jxact_id) {
//             None => return false,
//             Some(value) => &mut value.list,
//         };
//         let xact_lines_copy = xact_lines.clone();

//         let mut entry_list = HashMap::<LedgerKey, ledger_line::ActiveModel>::new();
//         let mut ledger_xact_list = HashMap::<LedgerKey, transaction::ledger::ActiveModel>::new();
//         let mut ledger_posted_list =
//             Vec::<(LedgerKey, &journal_transaction_line_ledger::ActiveModel)>::new();
//         let cr_xact_lines = xact_lines_copy
//             .iter()
//             .filter(|am| am.xact_type == XactType::Cr)
//             .collect::<Vec<_>>();
//         let dr_xact_lines = xact_lines_copy
//             .iter()
//             .filter(|am| am.xact_type == XactType::Dr)
//             .collect::<Vec<_>>();
//         for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
//             let key = LedgerKey {
//                 ledger_id: cr.ledger_id,
//                 timestamp: cr.timestamp,
//             };
//             let entry = ledger_line::ActiveModel {
//                 ledger_id: key.ledger_id,
//                 timestamp: key.timestamp,
//                 ledger_xact_type_code: ledger_xact_type.code,
//                 amount: cr.amount,
//                 journal_ref: jxact_id,
//             };
//             let tx_dr = transaction::ledger::ActiveModel {
//                 ledger_id: key.ledger_id,
//                 timestamp: key.timestamp,
//                 ledger_dr_id: dr.ledger_id,
//             };
//             entry_list.insert(key, entry);
//             ledger_xact_list.insert(key, tx_dr);
//             ledger_posted_list.push((key, dr));
//             ledger_posted_list.push((key, cr));
//         }

//         for line in xact_lines.iter_mut() {
//             for (key, post_line) in ledger_posted_list.iter() {
//                 if *line == **post_line {
//                     line.state = TransactionState::Posted;
//                     line.posting_ref = Some(PostingRef {
//                         key: *key,
//                         account_id: line.ledger_id,
//                     });
//                 }
//             }
//         }
//         for (k, e) in entry_list.iter() {
//             inner.journal_entry.insert(*k, *e);
//         }
//         for (k, tx_dr) in ledger_xact_list.iter() {
//             inner.ledger_xact.insert(*k, *tx_dr);
//         }

//         true
//     }

//     async fn get_journal_entry_type(
//         &self,
//         _jxact_id: JournalTransactionId,
//     ) -> ledger_xact_type::ActiveModel {
//         let inner = self.inner.read().await;

//         *inner
//             .ledger_xact_type
//             .get(&LedgerXactTypeCode::from_str("AL").unwrap())
//             .unwrap()
//     }
// }

// impl AccountEngineStorage for MemoryStore {
// fn journal_transactions_by_ledger(&self, ledger_name: &LedgerId) -> Vec<JournalTransaction> {
//     let mut res = Vec::<JournalTransaction>::new();
//     let inner = self.inner.read().unwrap();
//     for value in inner.journal_txs.values() {
//         if value.journal.ledger == *ledger_name {
//             res.insert(0, value.clone());
//         }
//     }

//     res
// }

// fn journal_transaction_by_id(&self, id: JournalTransactionId) -> Option<JournalTransaction> {
//     let inner = self.inner.read().unwrap();
//     for value in inner.journal_txs.values() {
//         if value.id == id {
//             return Some(value.clone());
//         }
//     }

//     None
// }
// }
