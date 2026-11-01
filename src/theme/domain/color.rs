use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ColorError {
    #[error("color must be a 3- or 6-digit hex color (received {value:?})")]
    InvalidHex { value: String },
}

/// Result for parsing palette colors.
pub type ColorResult<T> = Result<T, ColorError>;

impl TryFrom<String> for Color {
    type Error = ColorError;

    fn try_from(value: String) -> ColorResult<Self> {
        let valid = value.strip_prefix('#').is_some_and(|hex| {
            matches!(hex.len(), 3 | 6) && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
        });

        if !valid {
            return Err(ColorError::InvalidHex { value });
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorError;

    fn try_from(value: &str) -> ColorResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl Color {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
