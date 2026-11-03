use crate::{
    preferences::application::ports::{PreferencesReader, PreferencesWriter},
    shared::{application::ApplicationResult, domain::UserId},
    theme::{
        application::{
            commands::DeleteTheme,
            ports::{ThemeReader, ThemeWriter},
        },
        domain::ThemeId,
    },
};

/// Handles `DELETE /api/v1/themes/{theme_id}`.
pub async fn handle<P, T>(
    preferences: P,
    themes: T,
    owner: &UserId,
    id: ThemeId,
) -> ApplicationResult<()>
where
    P: PreferencesReader + PreferencesWriter,
    T: ThemeReader + ThemeWriter,
{
    DeleteTheme::new(preferences, themes)
        .execute(owner, id)
        .await
}
