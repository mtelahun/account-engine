use std::ops::Deref;

use arrayvec::ArrayString;
use postgres_types::{to_sql_checked, FromSql, ToSql};

use super::{fixed_len_char::InvalidLengthError, STRING64_LEN};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct ArrayString128(ArrayString<STRING64_LEN>);

impl ArrayString128 {
    pub const LENGTH: usize = STRING64_LEN;

    pub fn new() -> Self {
        ArrayString128(ArrayString::<STRING64_LEN>::new())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.0.is_full()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for ArrayString128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0.as_str())
    }
}

impl std::str::FromStr for ArrayString128 {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let inner = ArrayString::<STRING64_LEN>::from_str(src).map_err(|_| InvalidLengthError {
            expected: STRING64_LEN,
            actual: src.len(),
        })?;

        Ok(Self(inner))
    }
}

impl From<&str> for ArrayString128 {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for ArrayString128 {
    fn from(value: String) -> Self {
        let mut value = value;
        if value.len() > STRING64_LEN {
            value.truncate(STRING64_LEN);
        }

        Self(ArrayString::<STRING64_LEN>::from(&value).unwrap())
    }
}

impl From<ArrayString<STRING64_LEN>> for ArrayString128 {
    fn from(value: ArrayString<STRING64_LEN>) -> Self {
        Self(value)
    }
}

impl Deref for ArrayString128 {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<ArrayString128> for str {
    fn eq(&self, other: &ArrayString128) -> bool {
        *self == *other.as_str()
    }
}

impl AsRef<str> for ArrayString128 {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> FromSql<'a> for ArrayString128 {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let res = <&str as FromSql>::from_sql(ty, raw).map(ToString::to_string)?;

        Ok(ArrayString128::from(res))
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for ArrayString128 {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        <&str as ToSql>::to_sql(&&**self, ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
