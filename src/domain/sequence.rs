use postgres_types::{FromSql, ToSql};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, ToSql, FromSql)]
#[postgres(name = "sequence")]
pub struct Sequence(i16);

impl Sequence {
    pub fn new(value: i16) -> Option<Sequence> {
        if value > 0 {
            Some(Sequence(value))
        } else {
            None
        }
    }
}

impl std::ops::Deref for Sequence {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i16> for Sequence {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Sequence(value))
        } else {
            Err("only values between 1 and 32,767 are accepted")
        }
    }
}

impl std::fmt::Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
