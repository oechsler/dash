use std::collections::HashMap;

use crate::{
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId, ThemeMode},
    transfer::application::models::formats::v2::SettingsExport,
    user::domain::UserId,
};

pub(super) fn build(
    owner: &UserId,
    settings: SettingsExport,
    themes: &[Theme],
    theme_ids: &HashMap<(String, ThemeMode), ThemeId>,
) -> Result<UserPreferences, ApplicationError> {
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.set_language(parse_language(&settings.language)?);
    preferences.set_time_zone(TimeZone::parse(settings.time_zone)?);
    select_theme(
        &mut preferences,
        ThemeMode::Light,
        settings.light_theme.as_deref(),
        themes,
        theme_ids,
    )?;
    select_theme(
        &mut preferences,
        ThemeMode::Dark,
        settings.dark_theme.as_deref(),
        themes,
        theme_ids,
    )?;
    Ok(preferences)
}

fn select_theme(
    preferences: &mut UserPreferences,
    mode: ThemeMode,
    name: Option<&str>,
    themes: &[Theme],
    theme_ids: &HashMap<(String, ThemeMode), ThemeId>,
) -> Result<(), ApplicationError> {
    let Some(name) = name else { return Ok(()) };
    let Some(id) = theme_ids.get(&(name.to_owned(), mode)) else {
        return Ok(());
    };
    let theme = themes
        .iter()
        .find(|theme| theme.id() == *id)
        .ok_or(ApplicationError::NotFound("theme"))?;
    preferences.select_theme(theme)?;
    Ok(())
}

fn parse_language(value: &str) -> Result<Language, ApplicationError> {
    match value {
        "" | "auto" => Ok(Language::Auto),
        "en" => Ok(Language::English),
        "de" => Ok(Language::German),
        _ => Err(
            crate::transfer::application::error::TransferError::InvalidDocument(format!(
                "unknown language `{value}`"
            ))
            .into(),
        ),
    }
}
