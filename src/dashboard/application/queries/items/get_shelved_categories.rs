use crate::shared::application::ApplicationResult;
use crate::{
    dashboard::{application::ports::DashboardReader, domain::ShelvedCategory},
    shared::domain::UserId,
};

pub struct GetUserShelvedCategories<R> {
    repository: R,
}

impl<R: DashboardReader> GetUserShelvedCategories<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> ApplicationResult<Vec<ShelvedCategory>> {
        let Some(dashboard) = self.repository.load(owner).await? else {
            return Ok(Vec::new());
        };
        Ok(dashboard
            .categories()
            .iter()
            .filter_map(|category| category.as_shelved().cloned())
            .collect())
    }
}
