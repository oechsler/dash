use crate::shared::application::ApplicationResult;
use crate::{
    session::{application::ports::SessionReader, domain::UserSession},
    shared::application::ApplicationError,
    shared::domain::UserId,
};

pub struct ListSessions<R> {
    repository: R,
}

impl<R: SessionReader> ListSessions<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId) -> ApplicationResult<Vec<UserSession>> {
        self.repository
            .list_for(owner)
            .await
            .map_err(ApplicationError::from)
    }
}
