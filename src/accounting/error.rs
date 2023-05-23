#[derive(Debug, PartialEq)]
pub enum AccountError {
    ValidationError(String),
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AccountError::ValidationError(msg) = self;
        write!(f, "ValidationError: {}", msg)
    }
}

impl std::error::Error for AccountError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        let err =
            AccountError::ValidationError("some error validating caller supplied input".into());

        assert_eq!(
            err.to_string(),
            "ValidationError: some error validating caller supplied input",
            "error string has correct format",
        )
    }
}
