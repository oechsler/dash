use async_trait::async_trait;

use crate::{
    session::domain::{SessionId, UserSession},
    shared::application::ApplicationPortError,
    shared::domain::UserId,
};

#[async_trait]
pub trait SessionReader: Send + Sync {
    async fn load(
        &self,
        owner: &UserId,
        id: SessionId,
    ) -> Result<Option<UserSession>, ApplicationPortError>;
    async fn list_for(&self, owner: &UserId) -> Result<Vec<UserSession>, ApplicationPortError>;
}
