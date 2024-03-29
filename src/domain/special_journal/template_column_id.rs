use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "templatecolumnid")]
pub struct TemplateColumnId(uuid::Uuid);

impl TemplateColumnId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<TemplateColumnId, String> {
        let res = uuid::Uuid::parse_str(input)
            .map_err(|e| format!("unable to parse JournalId: {}", e))?;

        Ok(Self(res))
    }
}

impl std::fmt::Display for TemplateColumnId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TemplateColumnId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
