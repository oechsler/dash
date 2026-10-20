use super::{IdentifierError, UserError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: impl Into<String>) -> Result<Self, UserError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(IdentifierError::EmptyUserId { value }.into());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
