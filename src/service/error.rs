use crate::resource::OrmError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceError {
    EmptyRecord(String),
    Resource(OrmError),
    Unknown(String),
    Validation(String),
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::EmptyRecord(_) => None,
            ServiceError::Resource(e) => Some(e),
            ServiceError::Unknown(_) => None,
            ServiceError::Validation(_) => None,
        }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ServiceError::EmptyRecord(s) => format!("record does not exist: {s}"),
            ServiceError::Resource(e) => format!("resource failed: {e}"),
            ServiceError::Unknown(s) => format!("unknown error occurred: {s}"),
            ServiceError::Validation(s) => format!("validation error: {s}"),
        };

        write!(f, "service error: {}", msg)
    }
}

impl From<OrmError> for ServiceError {
    fn from(value: OrmError) -> Self {
        ServiceError::Resource(value)
    }
}
