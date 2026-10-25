use crate::{
    catalog::{
        application::ports::{ApplicationCatalogSettingsReader, ApplicationReader},
        domain::{Application, ApplicationCatalogSettings},
    },
    preferences::application::ports::PreferencesReader,
    shared::{application::ApplicationError, domain::CollectionSortOrder},
    user::domain::UserId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCatalogView {
    pub sort_order: CollectionSortOrder,
    pub applications: Vec<Application>,
}

pub struct ListApplications<A, S, P> {
    applications: A,
    settings: S,
    preferences: P,
}

impl<A, S, P> ListApplications<A, S, P>
where
    A: ApplicationReader,
    S: ApplicationCatalogSettingsReader,
    P: PreferencesReader,
{
    pub fn new(applications: A, settings: S, preferences: P) -> Self {
        Self {
            applications,
            settings,
            preferences,
        }
    }

    pub async fn execute(
        &self,
        user_id: &UserId,
    ) -> Result<ApplicationCatalogView, ApplicationError> {
        let settings = self
            .settings
            .load()
            .await?
            .unwrap_or_else(ApplicationCatalogSettings::default);
        let preferences = self.preferences.load(user_id).await?.unwrap_or_else(|| {
            crate::preferences::domain::UserPreferences::for_user(user_id.clone())
        });
        let mut applications = self.applications.list().await?;
        settings.sort_applications(
            &mut applications,
            preferences.recently_accessed_application_ids(),
        );
        Ok(ApplicationCatalogView {
            sort_order: settings.sort_order(),
            applications,
        })
    }
}
