use async_trait::async_trait;

use crate::{shared::application::ApplicationError, user::domain::UserId};

#[async_trait]
pub trait UserWriter: Send + Sync {
    /// Deletes the user and all owned data transactionally.
    async fn delete(&self, user_id: &UserId) -> Result<(), ApplicationError>;
}
