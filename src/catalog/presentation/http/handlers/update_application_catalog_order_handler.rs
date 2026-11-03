use crate::{
    catalog::application::{
        commands::{UpdateApplicationCatalogOrder, UpdateApplicationCatalogOrderInput},
        ports::{
            ApplicationCatalogSettingsReader, ApplicationCatalogSettingsWriter, ApplicationReader,
        },
    },
    shared::application::ApplicationResult,
};

/// Handles `PUT /api/v1/applications/order`.
pub async fn handle<A, S>(
    applications: A,
    settings: S,
    request: &dash_api::models::UpdateApplicationCatalogOrderRequest,
) -> ApplicationResult<()>
where
    A: ApplicationReader,
    S: ApplicationCatalogSettingsReader + ApplicationCatalogSettingsWriter,
{
    UpdateApplicationCatalogOrder::new(applications, settings)
        .execute(UpdateApplicationCatalogOrderInput {
            application_ids: request
                .application_ids
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
        })
        .await
}
