use crate::{
    dashboard::{
        application::ports::DashboardReader,
        domain::{CategoryId, DashboardCategory},
    },
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct GetUserCategory<R> {
    repository: R,
}

impl<R: DashboardReader> GetUserCategory<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        owner: &UserId,
        id: CategoryId,
    ) -> Result<DashboardCategory, ApplicationError> {
        self.repository
            .load(owner)
            .await?
            .and_then(|dashboard| {
                dashboard
                    .categories()
                    .iter()
                    .find(|category| category.id() == id)
                    .cloned()
            })
            .ok_or(ApplicationError::NotFound("category"))
    }
}
