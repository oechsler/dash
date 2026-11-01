use std::str::FromStr;

use super::{PreferencesError, TimeZoneError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum TimeZone {
    #[default]
    Auto,
    Iana(chrono_tz::Tz),
}

impl TryFrom<String> for TimeZone {
    type Error = PreferencesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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
}

impl TryFrom<&str> for TimeZone {
    type Error = PreferencesError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
