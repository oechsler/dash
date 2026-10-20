use super::ThemeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalThemeId(&'static str);

impl GlobalThemeId {
    pub fn new(value: &'static str) -> Result<Self, ThemeError> {
        if value.trim().is_empty() {
            return Err(ThemeError::EmptyGlobalId);
        }
        Ok(Self(value))
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}
