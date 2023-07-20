use std::ops::Deref;

use arrayvec::ArrayString;
use postgres_types::{to_sql_checked, FromSql, ToSql};

use super::{fixed_len_char::InvalidLengthError, DEFAULT_LONGSTRING_LEN};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct ArrayLongString(ArrayString<DEFAULT_LONGSTRING_LEN>);

impl ArrayLongString {
    pub const LENGTH: usize = DEFAULT_LONGSTRING_LEN;

    pub fn new() -> Self {
        ArrayLongString(ArrayString::<DEFAULT_LONGSTRING_LEN>::new())
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

impl std::fmt::Display for ArrayLongString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0.as_str())
    }
}

impl std::str::FromStr for ArrayLongString {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let inner = ArrayString::<DEFAULT_LONGSTRING_LEN>::from_str(src).map_err(|_| {
            InvalidLengthError {
                expected: DEFAULT_LONGSTRING_LEN,
                actual: src.len(),
            }
        })?;

        Ok(Self(inner))
    }
}

impl From<&str> for ArrayLongString {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for ArrayLongString {
    fn from(value: String) -> Self {
        let mut value = value;
        if value.len() > DEFAULT_LONGSTRING_LEN {
            value.truncate(DEFAULT_LONGSTRING_LEN);
        }

        Self(ArrayString::<DEFAULT_LONGSTRING_LEN>::from(&value).unwrap())
    }
}

impl From<ArrayString<DEFAULT_LONGSTRING_LEN>> for ArrayLongString {
    fn from(value: ArrayString<DEFAULT_LONGSTRING_LEN>) -> Self {
        Self(value)
    }
}

impl Deref for ArrayLongString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<ArrayLongString> for str {
    fn eq(&self, other: &ArrayLongString) -> bool {
        *self == *other.as_str()
    }
}

impl PartialEq<str> for ArrayLongString {
    fn eq(&self, other: &str) -> bool {
        self == other
    }
}

impl<'a> FromSql<'a> for ArrayLongString {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let res = <&str as FromSql>::from_sql(ty, raw).map(ToString::to_string)?;

        Ok(ArrayLongString::from(res))
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for ArrayLongString {
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
