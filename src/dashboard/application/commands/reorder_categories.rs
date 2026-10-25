use crate::{
    dashboard::{
        application::ports::{DashboardReader, DashboardWriter},
        domain::{CategoryId, UserDashboard},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct ReorderCategoriesInput {
    pub owner: UserId,
    pub category_ids: Vec<CategoryId>,
}

pub struct ReorderCategories<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> ReorderCategories<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: ReorderCategoriesInput) -> Result<(), ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner.clone()));
        dashboard.reorder_categories(&input.category_ids)?;
        self.repository.save(&dashboard).await
    }
}
