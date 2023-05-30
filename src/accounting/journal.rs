use crate::domain::LedgerId;

use super::JournalTransaction;

#[derive(Clone, Debug)]
pub struct Journal {
    pub name: String,
    pub code: String,
    pub ledger: LedgerId,
    pub xacts: Vec<JournalTransaction>,
}
