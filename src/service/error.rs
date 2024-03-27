use crate::store::OrmError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceError {
    EmptyRecord(String),
    DuplicateResource(OrmError),
    ResourceNotFound(OrmError),
    Resource(OrmError),
    Unknown(String),
    Validation(String),
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ServiceError::EmptyRecord(_) => None,
            ServiceError::ResourceNotFound(e) => Some(e),
            ServiceError::DuplicateResource(e) => Some(e),
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
            ServiceError::DuplicateResource(e) => format!("duplicate resource: {e}"),
            ServiceError::ResourceNotFound(e) => format!("resource not found: {e}"),
            ServiceError::Resource(e) => format!("resource error: {e}"),
            ServiceError::Unknown(s) => format!("unknown error occurred: {s}"),
            ServiceError::Validation(s) => format!("validation error: {s}"),
        };

        write!(f, "service error: {}", msg)
    }
}

impl From<OrmError> for ServiceError {
    fn from(value: OrmError) -> Self {
        match value {
            OrmError::DuplicateRecord(_) => ServiceError::DuplicateResource(value),
            OrmError::RecordNotFound(_) => ServiceError::ResourceNotFound(value),
            _ => ServiceError::Resource(value),
        }
    }
}
