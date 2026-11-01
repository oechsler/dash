use crate::{
    catalog::domain::ApplicationId,
    shared::domain::UserId,
    theme::domain::{ThemeId, ThemeMode},
};
use getset::Getters;

use super::selected_themes::SelectedThemes;
use super::{Language, PreferencesError, TimeZone};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct UserPreferences {
    #[getset(get = "pub")]
    user_id: UserId,
    selected_themes: SelectedThemes,
    language: Language,
    recent_application_ids: Vec<ApplicationId>,
    #[getset(get = "pub")]
    time_zone: TimeZone,
}

impl UserPreferences {
    pub(crate) fn restore(
        user_id: UserId,
        light_theme: Option<ThemeId>,
        dark_theme: Option<ThemeId>,
        language: Language,
        recent_application_ids: Vec<ApplicationId>,
        time_zone: TimeZone,
    ) -> Self {
        let mut selected_themes = SelectedThemes::default();
        if let Some(id) = light_theme {
            selected_themes.select(ThemeMode::Light, id);
        }
        if let Some(id) = dark_theme {
            selected_themes.select(ThemeMode::Dark, id);
        }
        Self {
            user_id,
            selected_themes,
            language,
            recent_application_ids,
            time_zone,
        }
    }

    pub fn for_user(user_id: UserId) -> Self {
        Self {
            user_id,
            selected_themes: SelectedThemes::default(),
            language: Language::Auto,
            recent_application_ids: Vec::new(),
            time_zone: TimeZone::Auto,
        }
    }

    pub fn theme_id(&self, mode: ThemeMode) -> Option<ThemeId> {
        self.selected_themes.get(mode)
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn recently_accessed_application_ids(&self) -> &[ApplicationId] {
        &self.recent_application_ids
    }

    pub fn record_application_access(&mut self, id: ApplicationId) {
        self.recent_application_ids
            .retain(|existing| *existing != id);
        self.recent_application_ids.insert(0, id);
        self.recent_application_ids.truncate(100);
    }

    pub fn select_theme(&mut self, mode: ThemeMode, theme_id: ThemeId) {
        self.selected_themes.select(mode, theme_id);
    }

    /// Validates deletion against ownership, global scope, and both mode selections.
    pub fn ensure_theme_is_not_selected(&self, theme_id: ThemeId) -> Result<(), PreferencesError> {
        if self.uses_theme(theme_id.clone()) {
            return Err(PreferencesError::ThemeCurrentlySelected {
                theme_id,
                user_id: self.user_id.clone(),
            });
        }
        Ok(())
    }

    pub fn clear_theme_selection(&mut self, mode: ThemeMode) {
        self.selected_themes.clear(mode);
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn set_time_zone(&mut self, time_zone: TimeZone) {
        self.time_zone = time_zone;
    }

    pub fn uses_theme(&self, theme_id: ThemeId) -> bool {
        self.selected_themes.contains(theme_id)
    }
}
