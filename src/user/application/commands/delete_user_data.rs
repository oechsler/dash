use crate::user::domain::UserId;

use crate::{shared::application::ApplicationError, user::application::ports::UserWriter};

pub struct DeleteUserData<R> {
    repository: R,
}

impl<R: UserWriter> DeleteUserData<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: &UserId) -> Result<(), ApplicationError> {
        self.repository.delete(user_id).await
    }
}
