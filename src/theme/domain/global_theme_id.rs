use super::ThemeError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalThemeId(String);

impl GlobalThemeId {
    pub fn new(value: &'static str) -> Result<Self, ThemeError> {
        if value.trim().is_empty() {
            return Err(ThemeError::EmptyGlobalId);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for GlobalThemeId {
    type Error = ThemeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err(ThemeError::EmptyGlobalId);
        }
        Ok(Self(value.to_owned()))
    }
}
