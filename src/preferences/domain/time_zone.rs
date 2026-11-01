use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TimeZone {
    #[default]
    Auto,
    Iana(chrono_tz::Tz),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TimeZoneError {
    #[error("timezone must be `auto` or a valid IANA timezone (received {value:?})")]
    InvalidIanaName { value: String },
}

/// Result for parsing IANA timezone names.
pub type TimeZoneResult<T> = Result<T, TimeZoneError>;

impl TryFrom<String> for TimeZone {
    type Error = TimeZoneError;

    fn try_from(value: String) -> TimeZoneResult<Self> {
        if value == "auto" {
            return Ok(Self::Auto);
        }

        let timezone =
            chrono_tz::Tz::from_str(&value).map_err(|_| TimeZoneError::InvalidIanaName {
                value: value.clone(),
            })?;
        Ok(Self::Iana(timezone))
    }
}

impl TryFrom<&str> for TimeZone {
    type Error = TimeZoneError;

    fn try_from(value: &str) -> TimeZoneResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl TimeZone {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Iana(timezone) => timezone.name(),
        }
    }
}
