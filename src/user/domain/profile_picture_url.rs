use super::{ProfilePictureUrlError, UserError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfilePictureUrl(String);

impl TryFrom<String> for ProfilePictureUrl {
    type Error = UserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed =
            url::Url::parse(&value).map_err(|source| ProfilePictureUrlError::InvalidUrl {
                value: value.clone(),
                reason: source.to_string(),
            })?;

        if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
            return Err(ProfilePictureUrlError::InvalidUrl {
                value,
                reason: "expected an absolute HTTP(S) URL with a host".to_owned(),
            }
            .into());
        }

        Ok(Self(value))
    }
}

impl TryFrom<&str> for ProfilePictureUrl {
    type Error = UserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl ProfilePictureUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
