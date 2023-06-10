use super::unique_id::UniqueId;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct AccountBookId(UniqueId);

impl AccountBookId {
    pub fn new() -> AccountBookId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for AccountBookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct JournalId(UniqueId);

impl JournalId {
    pub fn new() -> JournalId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for JournalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct LedgerId(UniqueId);

impl LedgerId {
    pub fn new() -> LedgerId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PeriodId(UniqueId);

impl PeriodId {
    pub fn new() -> PeriodId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for PeriodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct InterimPeriodId(UniqueId);

impl InterimPeriodId {
    pub fn new() -> InterimPeriodId {
        Self(UniqueId::new())
    }
}

impl std::fmt::Display for InterimPeriodId {
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

    #[test]
    fn test_ledger_id() {
        let lid = LedgerId::new();
        assert_eq!(lid.to_string().len(), 36, "ledger ID is 36 chars long")
    }

    #[test]
    fn test_journal_id() {
        let jid = JournalId::new();
        assert_eq!(jid.to_string().len(), 36, "journal ID is 36 chars long")
    }

    #[test]
    fn test_period_id() {
        let acid = PeriodId::new();
        assert_eq!(acid.to_string().len(), 36, "period ID is 36 chars long")
    }

    #[test]
    fn test_interim_period_id() {
        let acid = InterimPeriodId::new();
        assert_eq!(
            acid.to_string().len(),
            36,
            "interim period ID is 36 chars long"
        )
    }
}
