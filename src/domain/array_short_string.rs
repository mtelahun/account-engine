use std::ops::Deref;

use arrayvec::ArrayString;
use postgres_types::{to_sql_checked, FromSql, ToSql};

use super::{fixed_len_char::InvalidLengthError, DEFAULT_SHORTSTRING_LEN};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct ArrayShortString(ArrayString<DEFAULT_SHORTSTRING_LEN>);

impl ArrayShortString {
    pub const LENGTH: usize = DEFAULT_SHORTSTRING_LEN;

    pub fn new() -> Self {
        ArrayShortString(ArrayString::new())
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

impl std::fmt::Display for ArrayShortString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl std::str::FromStr for ArrayShortString {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let inner = ArrayString::<DEFAULT_SHORTSTRING_LEN>::from_str(src).map_err(|_| {
            InvalidLengthError {
                expected: DEFAULT_SHORTSTRING_LEN,
                actual: 0,
            }
        })?;

        Ok(Self(inner))
    }
}

impl From<String> for ArrayShortString {
    fn from(value: String) -> Self {
        let mut value = value;
        if value.len() > DEFAULT_SHORTSTRING_LEN {
            value.truncate(DEFAULT_SHORTSTRING_LEN);
        }

        Self(ArrayString::<DEFAULT_SHORTSTRING_LEN>::from(&value).unwrap())
    }
}

impl From<ArrayString<DEFAULT_SHORTSTRING_LEN>> for ArrayShortString {
    fn from(value: ArrayString<DEFAULT_SHORTSTRING_LEN>) -> Self {
        Self(value)
    }
}

impl Deref for ArrayShortString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromSql<'a> for ArrayShortString {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let res = <&str as FromSql>::from_sql(ty, raw).map(ToString::to_string)?;

        Ok(ArrayShortString::from(res))
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for ArrayShortString {
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
