#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrmError {
    DuplicateRecord(String),
    Internal(String),
    RecordNotFound(String),
}

impl std::error::Error for OrmError {}

impl std::fmt::Display for OrmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            OrmError::DuplicateRecord(s) => format!("duplicate record: {}", s),
            OrmError::Internal(s) => format!("internal error: {}", s),
            OrmError::RecordNotFound(s) => format!("record not found: {}", s),
        };

        write!(f, "Orm Error: {}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        // Act
        let err = OrmError::DuplicateRecord(
            "uniqueness constraint violated when creating/updating a record".into(),
        );
        // Assert
        assert_eq!(
            err.to_string(),
            "Orm Error: duplicate record: uniqueness constraint violated when creating/updating a record",
            "error string has correct format",
        );

        // Act
        let err = OrmError::Internal("non-storage failure".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Orm Error: internal error: non-storage failure",
            "error string has correct format",
        );

        // Act
        let err = OrmError::RecordNotFound("record affer8979".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Orm Error: record not found: record affer8979",
            "error string has correct format",
        );
    }
}
