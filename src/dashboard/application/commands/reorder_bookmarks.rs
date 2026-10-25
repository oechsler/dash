use crate::{
    dashboard::{
        application::ports::{DashboardReader, DashboardWriter},
        domain::{BookmarkId, CategoryId},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct ReorderBookmarksInput {
    pub owner: UserId,
    pub category_id: CategoryId,
    pub bookmark_ids: Vec<BookmarkId>,
}

pub struct ReorderBookmarks<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> ReorderBookmarks<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: ReorderBookmarksInput) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .ok_or(ApplicationError::NotFound("dashboard"))?;
        let category = dashboard
            .category_mut(input.category_id)
            .ok_or(ApplicationError::NotFound("category"))?;
        category.reorder_bookmarks(&input.bookmark_ids)?;
        self.repository.save(&dashboard).await
    }
}
