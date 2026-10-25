use crate::{
    catalog::domain::ApplicationId,
    theme::domain::{Theme, ThemeId, ThemeMode},
    user::domain::UserId,
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

    pub fn select_theme(&mut self, theme: &Theme) -> Result<(), PreferencesError> {
        self.ensure_theme_available(theme)?;
        self.selected_themes.select(theme.mode(), theme.id());
        Ok(())
    }

    /// Validates deletion against ownership, global scope, and both mode selections.
    pub fn ensure_theme_can_be_deleted(&self, theme: &Theme) -> Result<(), PreferencesError> {
        self.ensure_theme_available(theme)?;
        if !theme.is_deletable() {
            return Err(PreferencesError::ThemeNotDeletable {
                theme_id: theme.id(),
            });
        }
        if self.uses_theme(theme.id()) {
            return Err(PreferencesError::ThemeCurrentlySelected {
                theme_id: theme.id(),
                user_id: self.user_id.clone(),
            });
        }
        Ok(())
    }

    fn ensure_theme_available(&self, theme: &Theme) -> Result<(), PreferencesError> {
        if theme.identity().is_available_to(&self.user_id) {
            return Ok(());
        }
        Err(PreferencesError::ThemeNotAvailableToUser {
            theme_id: theme.id(),
            user_id: self.user_id.clone(),
        })
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
