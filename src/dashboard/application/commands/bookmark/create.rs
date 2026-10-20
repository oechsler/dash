use crate::{
    dashboard::domain::{Bookmark, BookmarkId, CategoryId, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct CreateBookmarkInput {
    pub owner: UserId,
    pub category_id: CategoryId,
    pub name: String,
    pub icon: String,
    pub address: String,
}

pub struct CreateBookmark<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> CreateBookmark<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateBookmarkInput,
    ) -> Result<BookmarkId, ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner));
        let bookmark = Bookmark::new(
            input.name,
            IconReference::parse(input.icon)?,
            WebAddress::parse(input.address)?,
        )?;
        let id = bookmark.id();
        dashboard.add_bookmark(input.category_id, bookmark)?;
        self.repository.save(&dashboard).await?;
        Ok(id)
    }
}
