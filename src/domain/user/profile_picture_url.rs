use super::ProfilePictureUrlError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfilePictureUrl(String);

impl ProfilePictureUrl {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProfilePictureUrlError> {
        let value = value.into();
        let parsed =
            url::Url::parse(&value).map_err(|source| ProfilePictureUrlError::InvalidUrl {
                value: value.clone(),
                reason: source.to_string(),
            })?;

        if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
            return Err(ProfilePictureUrlError::InvalidUrl {
                value,
                reason: "expected an absolute HTTP(S) URL with a host".to_owned(),
            });
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
