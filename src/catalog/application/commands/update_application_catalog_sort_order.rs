use crate::{
    catalog::{
        application::ports::{ApplicationCatalogSettingsReader, ApplicationCatalogSettingsWriter},
        domain::ApplicationCatalogSettings,
    },
    shared::{application::ApplicationError, domain::CollectionSortOrder},
};

pub struct UpdateApplicationCatalogSortOrderInput {
    pub sort_order: CollectionSortOrder,
}

pub struct UpdateApplicationCatalogSortOrder<S> {
    settings: S,
}

impl<S> UpdateApplicationCatalogSortOrder<S>
where
    S: ApplicationCatalogSettingsReader + ApplicationCatalogSettingsWriter,
{
    pub fn new(settings: S) -> Self {
        Self { settings }
    }

    pub async fn execute(
        &self,
        input: UpdateApplicationCatalogSortOrderInput,
    ) -> Result<(), ApplicationError> {
        let mut settings = self
            .settings
            .load()
            .await?
            .unwrap_or_else(ApplicationCatalogSettings::default);
        settings.set_sort_order(input.sort_order);
        self.settings.save(&settings).await
    }
}
