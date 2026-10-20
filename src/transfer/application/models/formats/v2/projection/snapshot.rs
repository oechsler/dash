use crate::{
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
    theme::domain::{ThemeMode, ThemeScope},
    transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

use super::super::model::{SettingsExport, UserDataExport};
use super::{applications, categories, themes};

pub fn build(
    snapshot: UserDataSnapshot,
    owner: &UserId,
    username: String,
    include_applications: bool,
) -> Result<UserDataExport, ApplicationError> {
    let preferences = snapshot
        .preferences
        .unwrap_or_else(|| UserPreferences::for_user(owner.clone()));
    let themes = snapshot
        .themes
        .iter()
        .filter(|theme| matches!(theme.scope(), ThemeScope::UserDefined(_)))
        .map(themes::export)
        .collect();
    let light_theme = preferences
        .theme_id(ThemeMode::Light)
        .and_then(|id| themes::name_for(&snapshot.themes, id));
    let dark_theme = preferences
        .theme_id(ThemeMode::Dark)
        .and_then(|id| themes::name_for(&snapshot.themes, id));
    let categories = snapshot
        .dashboard
        .as_ref()
        .map(|dashboard| categories::export(dashboard.categories()))
        .unwrap_or_default();
    let applications = if include_applications {
        applications::export(&snapshot.applications)
    } else {
        Vec::new()
    };

    let mut export = UserDataExport {
        version: 2,
        exported_at: chrono::Utc::now().to_rfc3339(),
        username,
        settings: SettingsExport {
            language: preferences.language().as_str().to_owned(),
            time_zone: preferences.time_zone().as_str().to_owned(),
            light_theme,
            dark_theme,
        },
        themes,
        categories,
        applications,
        signature: None,
    };
    export.sign()?;
    Ok(export)
}
