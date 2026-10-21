use crate::{
    dashboard::{application::ports::DashboardReader, domain::Category},
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct GetUserCategories<R> {
    repository: R,
}

impl<R: DashboardReader> GetUserCategories<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> Result<Vec<Category>, ApplicationError> {
        let Some(dashboard) = self.repository.load(owner).await? else {
            return Ok(Vec::new());
        };
        Ok(dashboard
            .categories()
            .iter()
            .filter_map(|category| category.as_active().cloned())
            .collect())
    }
}
