use super::DomainResult;
use crate::shared::domain::UserId;
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
    pub(crate) fn restore_user_defined(
        id: uuid::Uuid,
        owner: UserId,
        name: String,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> DomainResult<Self> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(ThemeError::EmptyName { value: name });
        }
        Ok(Self {
            identity: ThemeIdentity::restore_user_defined(id, owner),
            appearance: ThemeAppearance::new(mode, palette),
            name: trimmed.to_owned(),
        })
    }

    #[builder]
    pub fn global(
        id: GlobalThemeId,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> DomainResult<Self> {
        Self::create(ThemeIdentity::Global(id), name, mode, palette)
    }

    #[builder]
    pub fn user_defined(
        owner: UserId,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> DomainResult<Self> {
        Self::create(ThemeIdentity::user_defined(owner), name, mode, palette)
    }

    fn create(
        identity: ThemeIdentity,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> DomainResult<Self> {
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

    pub fn ensure_deletable(&self) -> DomainResult<()> {
        if self.is_deletable() {
            return Ok(());
        }
        Err(ThemeError::NotDeletable {
            theme_id: self.id(),
        })
    }
}
