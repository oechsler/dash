use uuid::Uuid;

use super::{GlobalThemeId, ThemeError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeId {
    Global(GlobalThemeId),
    UserDefined(Uuid),
}

impl TryFrom<&str> for ThemeId {
    type Error = ThemeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(id) = value.strip_prefix("global:") {
            return Ok(Self::Global(GlobalThemeId::try_from(id)?));
        }
        if let Some(id) = value.strip_prefix("user:") {
            return id
                .parse()
                .map(Self::UserDefined)
                .map_err(|_| ThemeError::EmptyGlobalId);
        }
        Err(ThemeError::EmptyGlobalId)
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
