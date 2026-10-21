use crate::user::domain::UserId;
use bon::bon;

use super::{
    GlobalThemeId, ThemeAppearance, ThemeError, ThemeId, ThemeIdentity, ThemeMode, ThemePalette,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    identity: ThemeIdentity,
    appearance: ThemeAppearance,
    name: String,
}

#[bon]
impl Theme {
    #[builder]
    pub fn global(
        id: GlobalThemeId,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> Result<Self, ThemeError> {
        Self::create(ThemeIdentity::Global(id), name, mode, palette)
    }

    #[builder]
    pub fn user_defined(
        owner: UserId,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> Result<Self, ThemeError> {
        Self::create(ThemeIdentity::user_defined(owner), name, mode, palette)
    }

    fn create(
        identity: ThemeIdentity,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> Result<Self, ThemeError> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(ThemeError::EmptyName { value: name });
        }

        Ok(Self {
            identity,
            appearance: ThemeAppearance::new(mode, palette),
            name: trimmed.to_owned(),
        })
    }

    pub fn id(&self) -> ThemeId {
        self.identity.id()
    }

    pub fn identity(&self) -> &ThemeIdentity {
        &self.identity
    }

    pub fn mode(&self) -> ThemeMode {
        self.appearance.mode()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn palette(&self) -> &ThemePalette {
        self.appearance.palette()
    }

    pub fn is_deletable(&self) -> bool {
        self.identity.is_user_defined()
    }
}
