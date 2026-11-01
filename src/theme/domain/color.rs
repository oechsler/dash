use super::{ColorError, ThemeError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color(String);

impl TryFrom<String> for Color {
    type Error = ThemeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = value.strip_prefix('#').is_some_and(|hex| {
            matches!(hex.len(), 3 | 6) && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
        });

        if !valid {
            return Err(ColorError::InvalidHex { value }.into());
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for Color {
    type Error = ThemeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Color {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
