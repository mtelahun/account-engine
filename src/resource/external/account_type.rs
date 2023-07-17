use crate::domain::ExternalAccountTypeCode;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Model {
    pub code: ExternalAccountTypeCode,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ActiveModel {
    pub code: ExternalAccountTypeCode,
    pub description: &'static str,
}
