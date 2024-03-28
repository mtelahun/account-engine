use std::ops::Deref;

use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "entity")]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new() -> EntityId {
        Self(uuid::Uuid::new_v4())
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

#[cfg(test)]
mod tests {
    use crate::shared_kernel::entity_id::EntityId;

    #[test]
    fn test_entity_id() {
        let lid = EntityId::new();
        assert_eq!(lid.to_string().len(), 36, "Entity ID is 36 chars long")
    }
}
