use std::collections::HashMap;

use crate::{
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId, ThemeMode},
    transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub(super) fn merge(
    target: &mut UserDataSnapshot,
    owner: &UserId,
    imported: &UserDataSnapshot,
    theme_ids: &HashMap<ThemeId, ThemeId>,
) -> Result<(), ApplicationError> {
    let Some(imported) = imported.preferences.as_ref() else {
        return Ok(());
    };
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.set_language(imported.language());
    preferences.set_time_zone(imported.time_zone().clone());
    remap_theme(
        &mut preferences,
        ThemeMode::Light,
        imported,
        theme_ids,
        &target.themes,
    )?;
    remap_theme(
        &mut preferences,
        ThemeMode::Dark,
        imported,
        theme_ids,
        &target.themes,
    )?;
    target.preferences = Some(preferences);
    Ok(())
}

fn remap_theme(
    preferences: &mut UserPreferences,
    mode: ThemeMode,
    imported_preferences: &UserPreferences,
    theme_ids: &HashMap<ThemeId, ThemeId>,
    themes: &[Theme],
) -> Result<(), ApplicationError> {
    let Some(imported_id) = imported_preferences.theme_id(mode) else {
        return Ok(());
    };
    let Some(target_id) = theme_ids.get(&imported_id) else {
        return Ok(());
    };
    let theme = themes
        .iter()
        .find(|theme| theme.id() == *target_id)
        .ok_or(ApplicationError::NotFound("theme"))?;
    preferences.select_theme(theme)?;
    Ok(())
}
