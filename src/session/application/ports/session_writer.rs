use crate::shared::application::PortResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{
    session::domain::{SessionId, UserSession},
    shared::domain::UserId,
};

#[async_trait]
pub trait SessionWriter: Send + Sync {
    async fn save(&self, session: &UserSession) -> PortResult<()>;
    async fn delete(&self, owner: &UserId, id: SessionId) -> PortResult<()>;
    async fn delete_expired(&self, now: DateTime<Utc>) -> PortResult<()>;
}
