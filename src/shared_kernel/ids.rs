use std::ops::Deref;

use postgres_types::{FromSql, ToSql};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "accountid")]
pub struct AccountId(uuid::Uuid);

impl AccountId {
    pub fn new() -> Self {
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
#[postgres(name = "entityid")]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn parse_str(input: &str) -> Result<EntityId, String> {
        let res = uuid::Uuid::parse_str(input)
            .map_err(|e| format!("unable to parse JournalId: {}", e))?;

        Ok(Self(res))
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for EntityId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "subledgerid")]
pub struct SubLedgerId(uuid::Uuid);

impl SubLedgerId {
    pub fn new() -> SubLedgerId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for SubLedgerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SubLedgerId {
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

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "specialjournalcolid")]
pub struct SpecialJournalColId(uuid::Uuid);

impl SpecialJournalColId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for SpecialJournalColId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for SpecialJournalColId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "columntotalid")]
pub struct ColumnTotalId(uuid::Uuid);

impl ColumnTotalId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for ColumnTotalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ColumnTotalId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "journaltypeid")]
pub struct JournalTypeId(uuid::Uuid);

impl JournalTypeId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for JournalTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for JournalTypeId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "journalrefid")]
pub struct JournalRefId(uuid::Uuid);

impl JournalRefId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for JournalRefId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for JournalRefId {
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
