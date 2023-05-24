use std::sync::Arc;

use super::{JournalTransaction, Ledger};

#[derive(Clone, Debug)]
pub struct Journal {
    pub name: String,
    pub code: String,
    pub ledger: Arc<Ledger>,
    pub xacts: Vec<JournalTransaction>,
}
