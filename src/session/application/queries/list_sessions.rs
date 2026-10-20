use crate::{
    session::{application::ports::SessionReader, domain::UserSession},
    shared::application::ApplicationError,
    user::domain::UserId,
};

pub struct ListSessions<R> {
    repository: R,
}

impl<R: SessionReader> ListSessions<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> Result<Vec<UserSession>, ApplicationError> {
        self.repository.list_for(owner).await
    }
}
