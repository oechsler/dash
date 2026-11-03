use crate::{
    catalog::application::{
        ports::{ApplicationCatalogSettingsReader, ApplicationReader},
        queries::{ApplicationCatalogView, ListApplications},
    },
    preferences::application::ports::PreferencesReader,
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `GET /api/v1/applications`.
pub async fn handle<A, S, P>(
    applications: A,
    settings: S,
    preferences: P,
    user: &UserId,
) -> ApplicationResult<ApplicationCatalogView>
where
    A: ApplicationReader,
    S: ApplicationCatalogSettingsReader,
    P: PreferencesReader,
{
    ListApplications::new(applications, settings, preferences)
        .execute(user)
        .await
}
