use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct JournalSequence {
    inner: Uuid,
}

impl JournalSequence {
    pub fn new() -> JournalSequence {
        Self {
            inner: Uuid::new_v4(),
        }
    }
}

impl std::fmt::Display for JournalSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
