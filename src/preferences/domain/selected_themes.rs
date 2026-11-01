use crate::theme::domain::{ThemeId, ThemeMode};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct SelectedThemes {
    light: Option<ThemeId>,
    dark: Option<ThemeId>,
}

impl SelectedThemes {
    pub(super) fn get(&self, mode: ThemeMode) -> Option<ThemeId> {
        match mode {
            ThemeMode::Light => self.light.clone(),
            ThemeMode::Dark => self.dark.clone(),
        }
    }

    pub(super) fn select(&mut self, mode: ThemeMode, theme_id: ThemeId) {
        match mode {
            ThemeMode::Light => self.light = Some(theme_id),
            ThemeMode::Dark => self.dark = Some(theme_id),
        }
    }

    pub(super) fn clear(&mut self, mode: ThemeMode) {
        match mode {
            ThemeMode::Light => self.light = None,
            ThemeMode::Dark => self.dark = None,
        }
    }

    pub(super) fn contains(&self, theme_id: ThemeId) -> bool {
        self.light.as_ref() == Some(&theme_id) || self.dark.as_ref() == Some(&theme_id)
    }
}
