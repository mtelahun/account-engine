use postgres_types::{to_sql_checked, FromSql, ToSql};

use super::{fixed_len_char::InvalidLengthError, FixedLenChar};

const LEN: usize = 2;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ExternalAccountTypeCode {
    inner: FixedLenChar<LEN>,
}

impl ExternalAccountTypeCode {
    pub const LENGTH: usize = LEN;

    pub fn as_bytes(&self) -> [u8; LEN] {
        self.inner.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
        let res = ExternalAccountTypeCode::try_from(bytes)?;

        Ok(res)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl From<String> for ExternalAccountTypeCode {
    fn from(value: String) -> Self {
        Self {
            inner: FixedLenChar::<LEN>::from(value),
        }
    }
}

impl std::str::FromStr for ExternalAccountTypeCode {
    type Err = InvalidLengthError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let inner: FixedLenChar<LEN> = src.parse()?;

        Ok(Self { inner })
    }
}

impl TryFrom<&[u8]> for ExternalAccountTypeCode {
    type Error = InvalidLengthError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let inner = FixedLenChar::from_bytes(value)?;

        Ok(Self { inner })
    }
}

impl std::fmt::Display for ExternalAccountTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.as_str())
    }
}

impl std::ops::Deref for ExternalAccountTypeCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.inner.as_str()
    }
}

impl<'a> FromSql<'a> for ExternalAccountTypeCode {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let res = <&str as FromSql>::from_sql(ty, raw).map(ToString::to_string)?;

        Ok(ExternalAccountTypeCode::from(res))
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for ExternalAccountTypeCode {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() {
        let code: ExternalAccountTypeCode = ExternalAccountTypeCode::from_str("AA").unwrap();
        common_assert(code);
    }

    #[test]
    fn test_from_bytes() {
        let code = ExternalAccountTypeCode::from_bytes("AL".as_bytes()).unwrap();
        common_assert(code);
        assert_eq!(
            code.as_bytes(),
            "AL".as_bytes(),
            "Converting back as bytes is same as original string"
        )
    }

    #[test]
    fn test_try_from() {
        let code: ExternalAccountTypeCode =
            ExternalAccountTypeCode::try_from("AA".as_bytes()).unwrap();
        common_assert(code);
    }

    fn common_assert(code: ExternalAccountTypeCode) {
        assert_eq!(code.len(), LEN, "length of code is {LEN}");
        assert!(!code.is_empty(), "code is NOT empty");

        let str_id = code.as_str();
        assert_eq!(str_id.len(), LEN, "length of code str is {LEN}");

        assert_eq!(
            code.to_string(),
            str_id,
            "string and str versions of code are equal"
        );
    }
}
