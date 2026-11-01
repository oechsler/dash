use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAddress(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WebAddressError {
    #[error("web address is malformed: {value:?} ({reason})")]
    Malformed { value: String, reason: String },

    #[error("web address must include a host: {value:?}")]
    MissingHost { value: String },
}

/// Result for parsing web addresses.
pub type WebAddressResult<T> = Result<T, WebAddressError>;

impl TryFrom<String> for WebAddress {
    type Error = WebAddressError;

    fn try_from(value: String) -> WebAddressResult<Self> {
        let parsed = url::Url::parse(&value).map_err(|source| WebAddressError::Malformed {
            value: value.clone(),
            reason: source.to_string(),
        })?;
        if parsed.host().is_none() {
            return Err(WebAddressError::MissingHost { value });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for WebAddress {
    type Error = WebAddressError;

    fn try_from(value: &str) -> WebAddressResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl WebAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
