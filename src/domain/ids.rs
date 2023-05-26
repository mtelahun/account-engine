use super::unique_id::UniqueId;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct AccountId(UniqueId);

impl AccountId {
    pub fn new() -> AccountId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct JournalTransactionId(UniqueId);

impl JournalTransactionId {
    pub fn new() -> JournalTransactionId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for JournalTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ac_id() {
        let acid = AccountId::new();
        assert_eq!(acid.to_string().len(), 36, "account ID is 36 chars long")
    }

    #[test]
    fn test_jx_id() {
        let jxid = JournalTransactionId::new();
        assert_eq!(jxid.to_string().len(), 36, "journal tx ID is 36 chars long")
    }
}
