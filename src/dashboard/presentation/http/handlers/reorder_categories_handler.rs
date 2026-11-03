use crate::{
    dashboard::application::{
        commands::{ReorderCategories, ReorderCategoriesInput},
        ports::{DashboardReader, DashboardWriter},
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `PUT /api/v1/categories/order`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    request: &dash_api::models::ReorderCategoriesRequest,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    ReorderCategories::new(repository)
        .execute(ReorderCategoriesInput {
            owner,
            category_ids: request
                .category_ids
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
        })
        .await
}
