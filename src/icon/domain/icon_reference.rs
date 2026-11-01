use thiserror::Error;

/// Opaque icon identifier. The presentation layer decides how to resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IconReference(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IconReferenceError {
    #[error("icon reference must not be empty (received {value:?})")]
    Empty { value: String },
}

/// Result for parsing icon references.
pub type IconReferenceResult<T> = Result<T, IconReferenceError>;

impl TryFrom<String> for IconReference {
    type Error = IconReferenceError;

    fn try_from(value: String) -> IconReferenceResult<Self> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(IconReferenceError::Empty { value });
        }

        Ok(Self(trimmed.to_owned()))
    }
}

impl TryFrom<&str> for IconReference {
    type Error = IconReferenceError;

    fn try_from(value: &str) -> IconReferenceResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl IconReference {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
