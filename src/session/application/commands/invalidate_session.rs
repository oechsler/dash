use crate::{session::domain::SessionId, user::domain::UserId};

use crate::{session::application::ports::SessionWriter, shared::application::ApplicationError};

pub struct InvalidateSession<R> {
    repository: R,
}

impl<R: SessionWriter> InvalidateSession<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId, id: SessionId) -> Result<(), ApplicationError> {
        self.repository.delete(owner, id).await
    }
}
