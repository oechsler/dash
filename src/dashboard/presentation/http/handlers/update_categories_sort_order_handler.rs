use crate::{
    dashboard::application::{
        commands::{UpdateCategoriesSortOrder, UpdateCategoriesSortOrderInput},
        ports::{DashboardReader, DashboardWriter},
    },
    shared::{
        application::{ApplicationError, ApplicationResult},
        domain::{CollectionSortOrder, UserId},
    },
};

/// Handles `PUT /api/v1/categories`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    request: &dash_api::models::UpdateCategoriesSortOrderRequest,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    let sort_order =
        CollectionSortOrder::try_from(request.sort_order.as_str()).map_err(|error| {
            ApplicationError::infrastructure(
                "decode category sort order",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, error),
            )
        })?;
    UpdateCategoriesSortOrder::new(repository)
        .execute(UpdateCategoriesSortOrderInput { owner, sort_order })
        .await
}
