use crate::resource::journal::transaction::column;

use super::JournalTransactionColumn;

impl From<column::account_cr::ActiveModel> for JournalTransactionColumn {
    fn from(value: column::account_cr::ActiveModel) -> Self {
        JournalTransactionColumn::AccountCr(value)
    }
}

impl From<column::account_dr::ActiveModel> for JournalTransactionColumn {
    fn from(value: column::account_dr::ActiveModel) -> Self {
        JournalTransactionColumn::AccountDr(value)
    }
}

impl From<column::ledger_drcr::ActiveModel> for JournalTransactionColumn {
    fn from(value: column::ledger_drcr::ActiveModel) -> Self {
        JournalTransactionColumn::LedgerDrCr(value)
    }
}

impl From<column::text::ActiveModel> for JournalTransactionColumn {
    fn from(value: column::text::ActiveModel) -> Self {
        JournalTransactionColumn::Text(value)
    }
}
