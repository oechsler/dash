use crate::domain::theme::{Theme, ThemeId, ThemeMode};
use crate::domain::user::UserId;
use getset::Getters;

use super::selected_themes::SelectedThemes;
use super::{Language, PreferencesError, TimeZone};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct UserPreferences {
    #[getset(get = "pub")]
    user_id: UserId,
    selected_themes: SelectedThemes,
    language: Language,
    #[getset(get = "pub")]
    time_zone: TimeZone,
}

impl UserPreferences {
    pub fn for_user(user_id: UserId) -> Self {
        Self {
            user_id,
            selected_themes: SelectedThemes::default(),
            language: Language::Auto,
            time_zone: TimeZone::Auto,
        }
    }

    pub fn theme_id(&self, mode: ThemeMode) -> Option<ThemeId> {
        self.selected_themes.get(mode)
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn select_theme(&mut self, theme: &Theme) -> Result<(), PreferencesError> {
        if !theme.scope().is_available_to(&self.user_id) {
            return Err(PreferencesError::ThemeNotAvailableToUser {
                theme_id: theme.id(),
                user_id: self.user_id.clone(),
            });
        }
        self.selected_themes.select(theme.mode(), theme.id());
        Ok(())
    }

    /// Validates deletion against ownership, global scope, and both mode selections.
    pub fn ensure_theme_can_be_deleted(&self, theme: &Theme) -> Result<(), PreferencesError> {
        if !theme.scope().is_available_to(&self.user_id) {
            return Err(PreferencesError::ThemeNotAvailableToUser {
                theme_id: theme.id(),
                user_id: self.user_id.clone(),
            });
        }
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
