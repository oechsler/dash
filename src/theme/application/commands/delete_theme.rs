use crate::shared::application::ApplicationResult;
use crate::{
    preferences::{application::ports::PreferencesReader, domain::UserPreferences},
    shared::application::ApplicationError,
    shared::domain::UserId,
    theme::{
        application::ports::{ThemeReader, ThemeWriter},
        domain::ThemeId,
    },
};

pub struct DeleteTheme<P, T> {
    preferences: P,
    themes: T,
}

impl<P: PreferencesReader, T: ThemeReader + ThemeWriter> DeleteTheme<P, T> {
    pub fn new(preferences: P, themes: T) -> Self {
        Self {
            preferences,
            themes,
        }
    }

    pub async fn execute(&self, owner: &UserId, id: ThemeId) -> ApplicationResult<()> {
        let theme = self
            .themes
            .get_for(owner, id.clone())
            .await?
            .ok_or(ApplicationError::NotFound("theme"))?;
        let preferences = self
            .preferences
            .load(owner)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(owner.clone()));
        theme.ensure_deletable()?;
        preferences.ensure_theme_is_not_selected(theme.id())?;
        self.themes
            .delete(owner, id)
            .await
            .map_err(ApplicationError::from)
    }
}
