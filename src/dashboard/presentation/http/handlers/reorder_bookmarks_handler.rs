use crate::{
    dashboard::{
        application::{
            commands::{ReorderBookmarks, ReorderBookmarksInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::CategoryId,
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `PUT /api/v1/categories/{category_id}/bookmarks/order`.
pub async fn handle<R>(
    repository: R,
    owner: UserId,
    category_id: CategoryId,
    request: &dash_api::models::ReorderBookmarksRequest,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    ReorderBookmarks::new(repository)
        .execute(ReorderBookmarksInput {
            owner,
            category_id,
            bookmark_ids: request
                .bookmark_ids
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
        })
        .await
}
