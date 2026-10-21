use crate::{
    dashboard::domain::{Category, CategoryId, DashboardCategory, ShelvedCategory, UserDashboard},
    shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct CreateCategoryInput {
    pub owner: UserId,
    pub name: String,
    pub shelved: bool,
}

pub struct CreateCategory<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> CreateCategory<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateCategoryInput,
    ) -> Result<CategoryId, ApplicationError> {
        let mut dashboard = self
            .repository
            .load(&input.owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(input.owner));
        let category = Category::new(input.name)?;
        let category: DashboardCategory = if input.shelved {
            ShelvedCategory::from(category).into()
        } else {
            category.into()
        };
        let id = category.id();
        dashboard.add_category(category);
        self.repository.save(&dashboard).await?;
        Ok(id)
    }
}
