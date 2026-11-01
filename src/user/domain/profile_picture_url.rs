use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfilePictureUrl(String);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProfilePictureUrlError {
    #[error("invalid profile picture URL {value:?}: {reason}")]
    InvalidUrl { value: String, reason: String },
}

/// Result for parsing profile-picture URLs.
pub type ProfilePictureUrlResult<T> = Result<T, ProfilePictureUrlError>;

impl TryFrom<String> for ProfilePictureUrl {
    type Error = ProfilePictureUrlError;

    fn try_from(value: String) -> ProfilePictureUrlResult<Self> {
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
}

impl TryFrom<&str> for ProfilePictureUrl {
    type Error = ProfilePictureUrlError;

    fn try_from(value: &str) -> ProfilePictureUrlResult<Self> {
        Self::try_from(value.to_owned())
    }
}

impl ProfilePictureUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
