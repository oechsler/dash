use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::shared::domain::UserId;

#[async_trait]
pub trait UserWriter: Send + Sync {
    /// Deletes the user and all owned data transactionally.
    async fn delete(&self, user_id: &UserId) -> PortResult<()>;
}
