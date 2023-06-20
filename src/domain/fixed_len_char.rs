use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct FixedLenChar<const L: usize> {
    inner: [u8; L],
}

impl<const L: usize> FixedLenChar<L> {
    pub fn as_bytes(&self) -> [u8; L] {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.inner).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
        Ok(Self {
            inner: bytes.try_into().map_err(|_| InvalidLengthError {
                expected: L,
                actual: bytes.len(),
            })?,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<const L: usize> From<String> for FixedLenChar<L> {
    fn from(value: String) -> Self {
        let mut value = value;
        if value.len() > L {
            value.truncate(L);
        }
        let mut inner: [u8; L] = [0; L];
        let mut inner_p: &mut [u8] = &mut inner;
        inner_p.write_all(value.as_bytes()).unwrap();

        Self { inner }
    }
}

impl<const L: usize> std::str::FromStr for FixedLenChar<L> {
    type Err = InvalidLengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

impl<const L: usize> Serialize for FixedLenChar<L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer
            .serialize_str(std::str::from_utf8(&self.inner).map_err(serde::ser::Error::custom)?)
    }
}

impl<'de, const L: usize> Deserialize<'de> for FixedLenChar<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_bytes(String::deserialize(deserializer)?.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug)]
pub struct InvalidLengthError {
    pub expected: usize,
    pub actual: usize,
}

impl std::fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "invalid length for 'FixedLenChar', expected: {}, actual: {}",
            self.expected, self.actual
        )
    }
}

impl std::error::Error for InvalidLengthError {}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn flc_from_str() {
        let flc: FixedLenChar<4> = FixedLenChar::from_str("abcd").unwrap();
        assert_eq!(
            flc.as_str(),
            "abcd",
            "end string is same as starting string"
        );
    }

    #[test]
    fn source_length_and_definition_mismatch() {
        let str_flc = "123";
        let flc = FixedLenChar::<4>::from_bytes(str_flc.as_bytes());
        assert!(flc.is_err(), "length mismatch returns error value");
    }

    #[test]
    fn serialize_deserialize() {
        let str_id = "123";
        let flc = FixedLenChar::<3>::from_bytes(str_id.as_bytes()).unwrap();
        let json_payload = serde_json::json!({
            "id": flc,
        });
        let json_string = json_payload.to_string();
        assert_eq!(
            json_string, r#"{"id":"123"}"#,
            "char array is correctly serialized then deserialized"
        );
    }

    #[test]
    fn test_is_empty() {
        let flc: FixedLenChar<4> = FixedLenChar::from_str("abcd").unwrap();
        assert!(flc.len() > 0, "char value has a positive length");
        assert!(!flc.is_empty(), "char value is NOT empty");

        let flc: FixedLenChar<0> = FixedLenChar::from_str("").unwrap();
        assert!(flc.len() == 0, "char value has ZERO length");
        assert!(flc.is_empty(), "char value IS empty");
    }

    #[test]
    fn test_error_display() {
        let e = InvalidLengthError {
            expected: 5,
            actual: 10,
        };

        assert_eq!(
            e.to_string(),
            "invalid length for 'FixedLenChar', expected: 5, actual: 10",
            "Error string displays correctly"
        );
    }
}
