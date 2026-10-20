use super::{ColorError, ThemeError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color(String);

impl Color {
    pub fn parse(value: impl Into<String>) -> Result<Self, ThemeError> {
        let value = value.into();
        let valid = value.strip_prefix('#').is_some_and(|hex| {
            matches!(hex.len(), 3 | 6) && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
        });

        if !valid {
            return Err(ColorError::InvalidHex { value }.into());
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
