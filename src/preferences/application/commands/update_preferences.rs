use crate::{
    preferences::{
        application::ports::{PreferencesReader, PreferencesWriter},
        domain::{Language, TimeZone, UserPreferences},
    },
    shared::application::ApplicationError,
    theme::{
        application::ports::ThemeReader,
        domain::{ThemeId, ThemeMode},
    },
    user::domain::UserId,
};

pub struct UpdatePreferencesInput {
    pub owner: UserId,
    pub mode: ThemeMode,
    pub theme_id: Option<ThemeId>,
    pub language: Option<Language>,
    pub time_zone: Option<String>,
}

pub struct UpdatePreferences<P, T> {
    preferences: P,
    themes: T,
}

impl<P: PreferencesReader + PreferencesWriter, T: ThemeReader> UpdatePreferences<P, T> {
    pub fn new(preferences: P, themes: T) -> Self {
        Self {
            preferences,
            themes,
        }
    }

    pub async fn execute(&self, input: UpdatePreferencesInput) -> Result<(), ApplicationError> {
        let mut preferences = self
            .preferences
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(input.owner.clone()));
        match input.theme_id {
            Some(id) => {
                let theme = self
                    .themes
                    .get_for(&input.owner, id)
                    .await?
                    .ok_or(ApplicationError::NotFound("theme"))?;
                preferences.select_theme(&theme)?;
            }
            None => preferences.clear_theme_selection(input.mode),
        }
        if let Some(language) = input.language {
            preferences.set_language(language);
        }
        if let Some(time_zone) = input.time_zone {
            preferences.set_time_zone(TimeZone::parse(time_zone)?);
        }
        self.preferences.save(&preferences).await
    }
}
