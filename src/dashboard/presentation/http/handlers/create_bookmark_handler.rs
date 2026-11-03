use crate::{
    dashboard::{
        application::{
            commands::{CreateBookmark, CreateBookmarkInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::{BookmarkId, CategoryId},
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `POST /api/v1/categories/{category_id}/bookmarks`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    category_id: CategoryId,
    request: &dash_api::models::CreateBookmarkRequest,
) -> ApplicationResult<BookmarkId>
where
    R: DashboardReader + DashboardWriter,
{
    CreateBookmark::new(repository)
        .execute(CreateBookmarkInput {
            owner,
            category_id,
            name: request.name.clone(),
            icon: request.icon.clone(),
            address: request.address.clone(),
        })
        .await
}
