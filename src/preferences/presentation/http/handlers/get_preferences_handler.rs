use crate::{
    preferences::{
        application::{ports::PreferencesReader, queries::GetPreferences},
        domain::UserPreferences,
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `GET /api/v1/preferences`.
pub async fn handle<R: PreferencesReader>(
    repository: R,
    owner: &UserId,
) -> ApplicationResult<UserPreferences> {
    GetPreferences::new(repository).execute(owner).await
}
