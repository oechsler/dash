use crate::{
    dashboard::domain::{BookmarkId, CategoryId, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct UpdateBookmarkInput {
    pub owner: UserId,
    pub bookmark_id: BookmarkId,
    pub category_id: CategoryId,
    pub name: String,
    pub icon: String,
    pub address: String,
}

pub struct UpdateBookmark<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> UpdateBookmark<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: UpdateBookmarkInput) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner));
        dashboard.move_bookmark(input.bookmark_id, input.category_id)?;
        let bookmark = dashboard.bookmark_mut(input.bookmark_id)?;
        bookmark.rename(input.name)?;
        bookmark.change_icon(IconReference::parse(input.icon)?);
        bookmark.change_address(WebAddress::parse(input.address)?);
        self.repository.save(&dashboard).await?;
        Ok(())
    }
}
