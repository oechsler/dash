use super::{IconError, IconReferenceError};

/// Opaque icon identifier. The presentation layer decides how to resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IconReference(String);

impl TryFrom<String> for IconReference {
    type Error = IconError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(IconReferenceError::Empty { value }.into());
        }

        Ok(Self(trimmed.to_owned()))
    }
}

impl TryFrom<&str> for IconReference {
    type Error = IconError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl IconReference {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
