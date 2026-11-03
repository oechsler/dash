use crate::{
    dashboard::{
        application::{
            commands::{CreateCategory, CreateCategoryInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::CategoryId,
    },
    shared::{
        application::ApplicationResult,
        domain::{CollectionSortOrder, UserId},
    },
};

/// Handles `POST /api/v1/categories`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    request: &dash_api::models::CreateCategoryRequest,
) -> ApplicationResult<CategoryId>
where
    R: DashboardReader + DashboardWriter,
{
    let sort_order = request
        .sort_order
        .as_deref()
        .map(CollectionSortOrder::try_from)
        .transpose()
        .map_err(invalid_sort_order)?;
    CreateCategory::new(repository)
        .execute(CreateCategoryInput {
            owner,
            name: request.name.clone(),
            shelved: request.shelved.unwrap_or(false),
            sort_order,
        })
        .await
}

fn invalid_sort_order(error: &'static str) -> crate::shared::application::ApplicationError {
    crate::shared::application::ApplicationError::infrastructure(
        "decode bookmark sort order",
        std::io::Error::new(std::io::ErrorKind::InvalidInput, error),
    )
}
