use crate::{
    dashboard::domain::{CategoryId, UserDashboard},
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct UpdateCategoryInput {
    pub owner: UserId,
    pub category_id: CategoryId,
    pub name: String,
    pub shelved: bool,
    pub bookmarks_sort_order: CollectionSortOrder,
}

pub struct UpdateCategory<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> UpdateCategory<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: UpdateCategoryInput) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner.clone()));
        dashboard.rename_category(input.category_id, input.name)?;
        if input.shelved {
            dashboard.shelve_category(input.category_id)?;
        } else {
            dashboard.unshelve_category(input.category_id)?;
        }
        dashboard
            .category_mut(input.category_id)
            .ok_or(ApplicationError::NotFound("category"))?
            .set_bookmarks_sort_order(input.bookmarks_sort_order);
        self.repository.save(&dashboard).await?;
        Ok(())
    }
}
