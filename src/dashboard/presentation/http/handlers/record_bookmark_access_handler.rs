use crate::{
    dashboard::{
        application::{
            commands::{RecordBookmarkAccess, RecordBookmarkAccessInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::{BookmarkId, CategoryId},
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `POST /api/v1/categories/{category_id}/bookmarks/{bookmark_id}/access`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    category_id: CategoryId,
    bookmark_id: BookmarkId,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    RecordBookmarkAccess::new(repository)
        .execute(RecordBookmarkAccessInput {
            owner,
            category_id,
            bookmark_id,
        })
        .await
}
