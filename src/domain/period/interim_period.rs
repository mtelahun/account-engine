use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

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
