#[derive(Debug, PartialEq)]
pub enum AccountError {
    Internal(String),
    Validation(String),
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AccountError::Internal(msg) => format!("Internal Error: {}", msg),
            AccountError::Validation(msg) => format!("Validation Error: {}", msg),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for AccountError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        // Act
        let err: AccountError =
            AccountError::Validation("some error validating caller supplied input".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Validation Error: some error validating caller supplied input",
            "error string has correct format",
        );

        // Act
        let err = AccountError::Internal("some internal library error".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Internal Error: some internal library error",
            "error string has correct format",
        );
    }
}
