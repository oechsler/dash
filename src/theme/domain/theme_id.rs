use thiserror::Error;
use uuid::Uuid;

use super::{GlobalThemeId, GlobalThemeIdError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeId {
    Global(GlobalThemeId),
    UserDefined(Uuid),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ThemeIdError {
    #[error("global theme ID is invalid: {0}")]
    Global(#[from] GlobalThemeIdError),

    #[error("user-defined theme ID is invalid: {value:?}")]
    InvalidUserDefined { value: String },

    #[error("theme ID must start with `global:` or `user:` (received {value:?})")]
    MissingNamespace { value: String },
}

/// Result for parsing namespaced theme identifiers.
pub type ThemeIdResult<T> = Result<T, ThemeIdError>;

impl TryFrom<&str> for ThemeId {
    type Error = ThemeIdError;

    fn try_from(value: &str) -> ThemeIdResult<Self> {
        if let Some(id) = value.strip_prefix("global:") {
            return Ok(Self::Global(GlobalThemeId::try_from(id)?));
        }
        if let Some(id) = value.strip_prefix("user:") {
            return id.parse().map(Self::UserDefined).map_err(|_| {
                ThemeIdError::InvalidUserDefined {
                    value: id.to_owned(),
                }
            });
        }
        Err(ThemeIdError::MissingNamespace {
            value: value.to_owned(),
        })
    }
}

impl From<ThemeId> for String {
    fn from(value: ThemeId) -> Self {
        match value {
            ThemeId::Global(id) => format!("global:{}", id.as_str()),
            ThemeId::UserDefined(id) => format!("user:{id}"),
        }
    }
}
