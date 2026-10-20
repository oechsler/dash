use super::WebAddressError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAddress(String);

impl WebAddress {
    pub fn parse(value: impl Into<String>) -> Result<Self, WebAddressError> {
        let value = value.into();
        let parsed = url::Url::parse(&value).map_err(|source| WebAddressError::Malformed {
            value: value.clone(),
            reason: source.to_string(),
        })?;
        if parsed.host().is_none() {
            return Err(WebAddressError::MissingHost { value });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
