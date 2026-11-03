use crate::{
    dashboard::{
        application::{
            commands::DeleteCategory,
            ports::{DashboardReader, DashboardWriter},
        },
        domain::CategoryId,
    },
    shared::{application::ApplicationResult, domain::UserId},
};

/// Handles `DELETE /api/v1/categories/{category_id}`.
pub async fn handle<R>(
    repository: R,
    owner: &UserId,
    category_id: CategoryId,
) -> ApplicationResult<()>
where
    R: DashboardReader + DashboardWriter,
{
    DeleteCategory::new(repository)
        .execute(owner, category_id)
        .await
}
