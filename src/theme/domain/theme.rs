use crate::user::domain::UserId;
use bon::bon;

use super::{GlobalThemeId, ThemeError, ThemeId, ThemeMode, ThemePalette, ThemeScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    id: ThemeId,
    scope: ThemeScope,
    mode: ThemeMode,
    name: String,
    palette: ThemePalette,
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
        Self::create(ThemeId::Global(id), ThemeScope::Global, name, mode, palette)
    }

    #[builder]
    pub fn user_defined(
        owner: UserId,
        name: impl Into<String>,
        mode: ThemeMode,
        palette: ThemePalette,
    ) -> Result<Self, ThemeError> {
        Self::create(
            ThemeId::user_defined(),
            ThemeScope::UserDefined(owner),
            name,
            mode,
            palette,
        )
    }

    fn create(
        id: ThemeId,
        scope: ThemeScope,
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
            id,
            scope,
            mode,
            name: trimmed.to_owned(),
            palette,
        })
    }

    pub fn id(&self) -> ThemeId {
        self.id
    }

    pub fn scope(&self) -> &ThemeScope {
        &self.scope
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn palette(&self) -> &ThemePalette {
        &self.palette
    }

    pub fn is_deletable(&self) -> bool {
        self.scope.is_user_defined()
    }
}
