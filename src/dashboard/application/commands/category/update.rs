use crate::{
    dashboard::domain::{CategoryId, UserDashboard},
    shared::application::ApplicationError,
    user::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct UpdateCategoryInput {
    pub owner: UserId,
    pub category_id: CategoryId,
    pub name: String,
    pub shelved: bool,
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
        self.repository.save(&dashboard).await?;
        Ok(())
    }
}
