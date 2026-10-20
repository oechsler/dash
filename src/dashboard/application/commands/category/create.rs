use crate::{
    dashboard::domain::{Category, CategoryId, UserDashboard},
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
        let mut category = Category::new(input.name)?;
        if input.shelved {
            category.shelve();
        }
        let id = category.id();
        dashboard.add_category(category);
        self.repository.save(&dashboard).await?;
        Ok(id)
    }
}
