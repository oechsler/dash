use crate::{
    preferences::{
        application::{
            commands::{UpdatePreferences, UpdatePreferencesInput},
            ports::{PreferencesReader, PreferencesWriter},
        },
        domain::Language,
    },
    shared::{
        application::{ApplicationError, ApplicationResult},
        domain::UserId,
    },
    theme::application::ports::ThemeReader,
};

/// Handles `PUT /api/v1/preferences`.
pub async fn handle<P, T>(
    preferences: P,
    themes: T,
    owner: UserId,
    request: &dash_api::models::UpdatePreferencesRequest,
) -> ApplicationResult<()>
where
    P: PreferencesReader + PreferencesWriter,
    T: ThemeReader,
{
    let language = Language::try_from(request.language.as_str()).map_err(|error| {
        ApplicationError::infrastructure(
            "decode language",
            std::io::Error::new(std::io::ErrorKind::InvalidInput, error),
        )
    })?;
    let light_theme_id = match &request.selected_themes.light {
        dash_api::types::Nullable::Null => None,
        dash_api::types::Nullable::Present(id) => {
            Some(crate::theme::domain::ThemeId::try_from(id.as_str())?)
        }
    };
    let dark_theme_id = match &request.selected_themes.dark {
        dash_api::types::Nullable::Null => None,
        dash_api::types::Nullable::Present(id) => {
            Some(crate::theme::domain::ThemeId::try_from(id.as_str())?)
        }
    };
    UpdatePreferences::new(preferences, themes)
        .execute(UpdatePreferencesInput {
            owner,
            language,
            time_zone: request.time_zone.clone(),
            light_theme_id,
            dark_theme_id,
        })
        .await
}
