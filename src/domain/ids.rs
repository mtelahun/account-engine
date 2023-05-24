use super::unique_id::UniqueId;

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
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
