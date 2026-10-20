use std::str::FromStr;

use super::TimeZoneError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TimeZone {
    #[default]
    Auto,
    Iana(String),
}

impl TimeZone {
    pub fn parse(value: impl Into<String>) -> Result<Self, TimeZoneError> {
        let value = value.into();
        if value == "auto" {
            return Ok(Self::Auto);
        }

        chrono_tz::Tz::from_str(&value).map_err(|_| TimeZoneError::InvalidIanaName {
            value: value.clone(),
        })?;
        Ok(Self::Iana(value))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Iana(value) => value,
        }
    }
}
