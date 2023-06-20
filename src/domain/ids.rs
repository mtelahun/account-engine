use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "accountid")]
pub struct AccountId(uuid::Uuid);

impl AccountId {
    pub fn new() -> AccountId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for AccountId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "accountbookid")]
pub struct AccountBookId(uuid::Uuid);

impl AccountBookId {
    pub fn new() -> AccountBookId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for AccountBookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for AccountBookId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "journalid")]
pub struct JournalId(uuid::Uuid);

impl JournalId {
    pub fn new() -> JournalId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for JournalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for JournalId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "generalledgerid")]
pub struct GeneralLedgerId(uuid::Uuid);

impl GeneralLedgerId {
    pub fn new() -> GeneralLedgerId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for GeneralLedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for GeneralLedgerId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "periodid")]
pub struct PeriodId(uuid::Uuid);

impl PeriodId {
    pub fn new() -> PeriodId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for PeriodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PeriodId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "interimperiodid")]
pub struct InterimPeriodId(uuid::Uuid);

impl InterimPeriodId {
    pub fn new() -> InterimPeriodId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for InterimPeriodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for InterimPeriodId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
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
    fn test_ledger_id() {
        let lid = GeneralLedgerId::new();
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
