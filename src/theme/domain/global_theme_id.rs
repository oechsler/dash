use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalThemeId(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GlobalThemeIdError {
    #[error("global theme ID must not be empty")]
    Empty,
}

/// Result for parsing global theme identifiers.
pub type GlobalThemeIdResult<T> = Result<T, GlobalThemeIdError>;

impl GlobalThemeId {
    pub fn new(value: &'static str) -> GlobalThemeIdResult<Self> {
        if value.trim().is_empty() {
            return Err(GlobalThemeIdError::Empty);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for GlobalThemeId {
    type Error = GlobalThemeIdError;

    fn try_from(value: &str) -> GlobalThemeIdResult<Self> {
        if value.trim().is_empty() {
            return Err(GlobalThemeIdError::Empty);
        }
        Ok(Self(value.to_owned()))
    }
}
