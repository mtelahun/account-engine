use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "ledgerid")]
pub struct LedgerId(uuid::Uuid);

impl LedgerId {
    pub fn new() -> LedgerId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for LedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for LedgerId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::general_ledger::ledger_id::LedgerId;

    #[test]
    fn test_ledger_id() {
        let acid = LedgerId::new();
        assert_eq!(acid.to_string().len(), 36, "ledger ID is 36 chars long")
    }
}
