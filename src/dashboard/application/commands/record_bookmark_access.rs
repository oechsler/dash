use chrono::Utc;

use crate::{
    dashboard::{
        application::ports::{DashboardReader, DashboardWriter},
        domain::{BookmarkId, CategoryId},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct RecordBookmarkAccessInput {
    pub owner: UserId,
    pub category_id: CategoryId,
    pub bookmark_id: BookmarkId,
}

pub struct RecordBookmarkAccess<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> RecordBookmarkAccess<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: RecordBookmarkAccessInput) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .ok_or(ApplicationError::NotFound("dashboard"))?;
        dashboard.record_bookmark_access(input.category_id, input.bookmark_id, Utc::now())?;
        self.repository.save(&dashboard).await
    }
}
