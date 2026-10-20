use super::IconReferenceError;

/// Opaque icon identifier. The presentation layer decides how to resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IconReference(String);

impl IconReference {
    pub fn parse(value: impl Into<String>) -> Result<Self, IconReferenceError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(IconReferenceError::Empty { value });
        }

        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
