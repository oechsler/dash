use crate::{
    catalog::{
        application::ports::{
            ApplicationCatalogSettingsReader, ApplicationCatalogSettingsWriter, ApplicationReader,
        },
        domain::{ApplicationCatalogSettings, ApplicationId},
    },
    shared::application::ApplicationError,
};

pub struct UpdateApplicationCatalogOrderInput {
    pub application_ids: Vec<ApplicationId>,
}

pub struct UpdateApplicationCatalogOrder<R, S> {
    applications: R,
    settings: S,
}

impl<R, S> UpdateApplicationCatalogOrder<R, S>
where
    R: ApplicationReader,
    S: ApplicationCatalogSettingsReader + ApplicationCatalogSettingsWriter,
{
    pub fn new(applications: R, settings: S) -> Self {
        Self {
            applications,
            settings,
        }
    }

    pub async fn execute(
        &self,
        input: UpdateApplicationCatalogOrderInput,
    ) -> Result<(), ApplicationError> {
        let apps = self.applications.list().await?;
        let mut settings = self
            .settings
            .load()
            .await?
            .unwrap_or_else(ApplicationCatalogSettings::default);
        let application_ids = apps.iter().map(|app| app.id()).collect::<Vec<_>>();
        settings.set_manual_order(input.application_ids, &application_ids)?;
        self.settings.save(&settings).await
    }
}
