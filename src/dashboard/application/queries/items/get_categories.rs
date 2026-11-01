use crate::shared::application::ApplicationResult;
use crate::{
    dashboard::{application::ports::DashboardReader, domain::Category},
    shared::domain::UserId,
};

pub struct GetUserCategories<R> {
    repository: R,
}

impl<R: DashboardReader> GetUserCategories<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> ApplicationResult<Vec<Category>> {
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
