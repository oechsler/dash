use std::str::FromStr;

use super::{PreferencesError, TimeZoneError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TimeZone {
    #[default]
    Auto,
    Iana(chrono_tz::Tz),
}

impl TimeZone {
    pub fn parse(value: impl Into<String>) -> Result<Self, PreferencesError> {
        let value = value.into();
        if value == "auto" {
            return Ok(Self::Auto);
        }

        let timezone = chrono_tz::Tz::from_str(&value).map_err(|_| {
            PreferencesError::from(TimeZoneError::InvalidIanaName {
                value: value.clone(),
            })
        })?;
        Ok(Self::Iana(timezone))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Iana(timezone) => timezone.name(),
        }
    }
}
