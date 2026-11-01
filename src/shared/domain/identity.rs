use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IdentityError {
    #[error("user ID must not be empty (received {value:?})")]
    EmptyUserId { value: String },

    #[error("group ID must not be empty (received {value:?})")]
    EmptyGroupId { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl TryFrom<String> for UserId {
    type Error = IdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(IdentityError::EmptyUserId { value });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for UserId {
    type Error = IdentityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl UserId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupId(String);

impl TryFrom<String> for GroupId {
    type Error = IdentityError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(IdentityError::EmptyGroupId { value });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for GroupId {
    type Error = IdentityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl GroupId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
