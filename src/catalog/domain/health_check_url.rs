use url::Url;

use super::CatalogApplicationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthCheckUrl(String);

impl TryFrom<String> for HealthCheckUrl {
    type Error = CatalogApplicationError;

    fn try_from(value: String) -> super::CatalogApplicationResult<Self> {
        let parsed =
            Url::parse(&value).map_err(|_| CatalogApplicationError::InvalidHealthCheckUrl {
                value: value.clone(),
            })?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
            return Err(CatalogApplicationError::InvalidHealthCheckUrl { value });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for HealthCheckUrl {
    type Error = CatalogApplicationError;

    fn try_from(value: &str) -> super::CatalogApplicationResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl HealthCheckUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
