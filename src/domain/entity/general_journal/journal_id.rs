use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "journalid")]
pub struct JournalId(uuid::Uuid);

impl JournalId {
    pub fn new() -> JournalId {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<JournalId, String> {
        let res = uuid::Uuid::parse_str(input)
            .map_err(|e| format!("unable to parse JournalId: {}", e))?;

        Ok(Self(res))
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
