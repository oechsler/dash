use crate::{
    dashboard::{
        application::{
            commands::{UpdateBookmark, UpdateBookmarkInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::{BookmarkId, CategoryId},
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `PUT /api/v1/categories/{category_id}/bookmarks/{bookmark_id}`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    category_id: CategoryId,
    bookmark_id: BookmarkId,
    request: &dash_api::models::CreateBookmarkRequest,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    UpdateBookmark::new(repository)
        .execute(UpdateBookmarkInput {
            owner,
            category_id,
            bookmark_id,
            name: request.name.clone(),
            icon: request.icon.clone(),
            address: request.address.clone(),
        })
        .await
}
