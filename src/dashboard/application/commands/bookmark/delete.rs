use crate::{
    dashboard::domain::{BookmarkId, UserDashboard},
    shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct DeleteBookmark<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> DeleteBookmark<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        bookmark_id: BookmarkId,
    ) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(owner.clone()));
        dashboard.remove_bookmark(bookmark_id)?;
        self.repository.save(&dashboard).await?;
        Ok(())
    }
}
