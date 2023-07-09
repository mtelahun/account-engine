#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrmError {
    Constraint(String),
    DuplicateRecord(String),
    Internal(String),
    RecordNotFound(String),
    Validation(String),
}

impl std::error::Error for OrmError {}

impl std::fmt::Display for OrmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            OrmError::Constraint(s) => format!("constraint violation: {}", s),
            OrmError::DuplicateRecord(s) => format!("duplicate record: {}", s),
            OrmError::Internal(s) => format!("internal error: {}", s),
            OrmError::RecordNotFound(s) => format!("record not found: {}", s),
            OrmError::Validation(s) => format!("validation error: {}", s),
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
        let err =
            OrmError::Constraint("one or more constraints on record creation was violated".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Orm Error: constraint violation: one or more constraints on record creation was violated",
            "error string has correct format",
        );

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

        // Act
        let err = OrmError::Validation("attempt to validate create/update failed".into());
        // Assert
        assert_eq!(
            err.to_string(),
            "Orm Error: validation error: attempt to validate create/update failed",
            "error string has correct format",
        );
    }
}
