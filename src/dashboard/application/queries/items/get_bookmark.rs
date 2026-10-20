use crate::{
    dashboard::{
        application::ports::DashboardReader,
        domain::{Bookmark, BookmarkId},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct GetUserBookmark<R> {
    repository: R,
}

impl<R: DashboardReader> GetUserBookmark<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        id: BookmarkId,
    ) -> Result<Bookmark, ApplicationError> {
        self.repository
            .load(owner)
            .await?
            .and_then(|dashboard| {
                dashboard.categories().iter().find_map(|category| {
                    category
                        .bookmarks()
                        .iter()
                        .find(|bookmark| bookmark.id() == id)
                        .cloned()
                })
            })
            .ok_or(ApplicationError::NotFound("bookmark"))
    }
}
