use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{
    session::domain::{SessionId, UserSession},
    shared::domain::UserId,
};

#[async_trait]
pub trait SessionReader: Send + Sync {
    async fn load(&self, owner: &UserId, id: SessionId) -> PortResult<Option<UserSession>>;
    async fn load_by_id(&self, id: SessionId) -> PortResult<Option<UserSession>>;
    async fn list_for(&self, owner: &UserId) -> PortResult<Vec<UserSession>>;
}
