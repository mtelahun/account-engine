use std::ops::Deref;

use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "specialjournaltemplateid")]
pub struct SpecialJournalTemplateId(uuid::Uuid);

impl SpecialJournalTemplateId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<SpecialJournalTemplateId, String> {
        let result = Uuid::parse_str(input);
        match result {
            Ok(uuid) => Ok(Self(uuid)),
            Err(e) => Err(format!("SpecialJournalTemplateId: {}", e)),
        }
    }
}

impl std::fmt::Display for SpecialJournalTemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SpecialJournalTemplateId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
