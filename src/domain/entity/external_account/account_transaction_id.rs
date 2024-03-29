use chrono::NaiveDateTime;

use super::account_id::AccountId;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountTransactionId(AccountId, NaiveDateTime);

impl AccountTransactionId {
    pub fn new(account_id: AccountId, dt: NaiveDateTime) -> Self {
        Self(account_id, dt)
    }

    pub fn account_id(&self) -> AccountId {
        self.0
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.1
    }
}

impl std::fmt::Display for AccountTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountTransactionId {{ Account ID: {}, Timestamp: {} }}",
            self.0, self.1
        )
    }
}
