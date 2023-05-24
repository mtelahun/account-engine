use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UniqueId {
    inner: Uuid,
}

impl UniqueId {
    pub fn new() -> Self {
        Self {
            inner: uuid::Uuid::new_v4(),
        }
    }
}

impl std::default::Default for UniqueId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for UniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let jid = UniqueId::new().to_string();
        assert_eq!(jid.len(), 36, "A uuid string is 36 chars long");
        assert_eq!(jid.matches('-').count(), 4, "A uuid string as 4 '-'s");
    }
}
