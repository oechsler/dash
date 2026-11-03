use crate::{
    dashboard::{
        application::{
            commands::DeleteBookmark,
            ports::{DashboardReader, DashboardWriter},
            queries::GetUserCategory,
        },
        domain::{BookmarkId, CategoryId},
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `DELETE /api/v1/categories/{category_id}/bookmarks/{bookmark_id}`.
pub async fn handle<R>(
    repository: R,
    owner: &UserId,
    category_id: CategoryId,
    bookmark_id: BookmarkId,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter + Clone,
{
    let category = GetUserCategory::new(repository.clone())
        .execute(owner, category_id)
        .await?;
    if !category
        .bookmarks()
        .iter()
        .any(|bookmark| bookmark.id() == bookmark_id)
    {
        return Err(crate::shared::application::ApplicationError::NotFound(
            "bookmark",
        ));
    }
    DeleteBookmark::new(repository)
        .execute(owner, bookmark_id)
        .await
}
