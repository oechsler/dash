use crate::shared::application::ApplicationResult;
use crate::shared::domain::UserId;

use crate::{shared::application::ApplicationError, user::application::ports::UserWriter};

pub struct DeleteUserData<R> {
    repository: R,
}

impl<R: UserWriter> DeleteUserData<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &UserId) -> ApplicationResult<()> {
        self.repository
            .delete(user_id)
            .await
            .map_err(ApplicationError::from)
    }
}
