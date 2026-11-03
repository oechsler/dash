use crate::{
    catalog::application::{
        commands::{UpdateApplicationCatalogSortOrder, UpdateApplicationCatalogSortOrderInput},
        ports::{ApplicationCatalogSettingsReader, ApplicationCatalogSettingsWriter},
    },
    shared::{
        application::{ApplicationError, ApplicationResult},
        domain::CollectionSortOrder,
    },
};

/// Handles `PUT /api/v1/applications`.
pub async fn handle<S>(
    settings: S,
    request: &dash_api::models::UpdateCategoriesSortOrderRequest,
) -> ApplicationResult<()>
where
    S: ApplicationCatalogSettingsReader + ApplicationCatalogSettingsWriter,
{
    let sort_order =
        CollectionSortOrder::try_from(request.sort_order.as_str()).map_err(|error| {
            ApplicationError::infrastructure(
                "decode application catalog sort order",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, error),
            )
        })?;
    UpdateApplicationCatalogSortOrder::new(settings)
        .execute(UpdateApplicationCatalogSortOrderInput { sort_order })
        .await
}
