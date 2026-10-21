use std::collections::HashMap;

use crate::{
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId},
    user::domain::UserId,
};

use super::super::model::GoSettingsV1;

pub(super) fn build(
    owner: &UserId,
    settings: GoSettingsV1,
    themes: &[Theme],
    theme_ids: &HashMap<String, ThemeId>,
) -> Result<UserPreferences, ApplicationError> {
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.set_language(parse_language(&settings.language)?);
    preferences.set_time_zone(TimeZone::parse(settings.timezone)?);
    if let Some(id) = settings
        .theme_name
        .and_then(|name| theme_ids.get(&name).copied())
    {
        let theme = themes
            .iter()
            .find(|theme| theme.id() == id)
            .ok_or(ApplicationError::NotFound("theme"))?;
        preferences.select_theme(theme)?;
    }
    Ok(preferences)
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
