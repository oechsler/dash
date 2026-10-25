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
    pub language: Language,
    pub time_zone: String,
    pub light_theme_id: Option<ThemeId>,
    pub dark_theme_id: Option<ThemeId>,
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
        preferences.set_language(input.language);
        preferences.set_time_zone(TimeZone::parse(input.time_zone)?);
        self.set_theme(
            &mut preferences,
            &input.owner,
            ThemeMode::Light,
            input.light_theme_id,
        )
        .await?;
        self.set_theme(
            &mut preferences,
            &input.owner,
            ThemeMode::Dark,
            input.dark_theme_id,
        )
        .await?;
        self.preferences.save(&preferences).await
    }

    async fn set_theme(
        &self,
        preferences: &mut UserPreferences,
        owner: &UserId,
        mode: ThemeMode,
        theme_id: Option<ThemeId>,
    ) -> Result<(), ApplicationError> {
        match theme_id {
            Some(id) => {
                let theme = self
                    .themes
                    .get_for(owner, id)
                    .await?
                    .ok_or(ApplicationError::NotFound("theme"))?;
                preferences.select_theme(&theme)?;
            }
            None => preferences.clear_theme_selection(mode),
        }
        Ok(())
    }
}
