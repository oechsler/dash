use url::Url;

use super::ApplicationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthCheckUrl(String);

impl HealthCheckUrl {
    pub fn parse(value: impl Into<String>) -> Result<Self, ApplicationError> {
        let value = value.into();
        let parsed = Url::parse(&value).map_err(|_| ApplicationError::InvalidHealthCheckUrl {
            value: value.clone(),
        })?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
            return Err(ApplicationError::InvalidHealthCheckUrl { value });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
