use crate::{
    dashboard::{
        application::{
            commands::{UpdateCategory, UpdateCategoryInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::CategoryId,
    },
    shared::{
        application::{ApplicationError, ApplicationResult},
        domain::{CollectionSortOrder, UserId},
    },
};

/// Handles `PUT /api/v1/categories/{category_id}`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    category_id: CategoryId,
    request: &dash_api::models::UpdateCategoryRequest,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    let sort_order =
        CollectionSortOrder::try_from(request.sort_order.as_str()).map_err(|error| {
            ApplicationError::infrastructure(
                "decode bookmark sort order",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, error),
            )
        })?;
    UpdateCategory::new(repository)
        .execute(UpdateCategoryInput {
            owner,
            category_id,
            name: request.name.clone(),
            shelved: request.shelved,
            sort_order,
        })
        .await
}
