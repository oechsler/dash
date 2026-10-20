use crate::{
    preferences::{application::ports::PreferencesReader, domain::UserPreferences},
    shared::application::ApplicationError,
    theme::{
        application::ports::{ThemeReader, ThemeWriter},
        domain::ThemeId,
    },
    user::domain::UserId,
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

    pub async fn execute(&self, owner: &UserId, id: ThemeId) -> Result<(), ApplicationError> {
        let theme = self
            .themes
            .get_for(owner, id)
            .await?
            .ok_or(ApplicationError::NotFound("theme"))?;
        let preferences = self
            .preferences
            .load(owner)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(owner.clone()));
        preferences.ensure_theme_can_be_deleted(&theme)?;
        self.themes.delete(owner, id).await
    }
}
