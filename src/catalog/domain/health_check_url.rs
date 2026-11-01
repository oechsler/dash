use url::Url;

use super::ApplicationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthCheckUrl(String);

impl TryFrom<String> for HealthCheckUrl {
    type Error = ApplicationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = Url::parse(&value).map_err(|_| ApplicationError::InvalidHealthCheckUrl {
            value: value.clone(),
        })?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
            return Err(ApplicationError::InvalidHealthCheckUrl { value });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for HealthCheckUrl {
    type Error = ApplicationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl HealthCheckUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
