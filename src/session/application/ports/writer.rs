use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{
    session::domain::{SessionId, UserSession},
    shared::application::ApplicationError,
    user::domain::UserId,
};

#[async_trait]
pub trait SessionWriter: Send + Sync {
    async fn save(&self, session: &UserSession) -> Result<(), ApplicationError>;
    async fn delete(&self, owner: &UserId, id: SessionId) -> Result<(), ApplicationError>;
    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<(), ApplicationError>;
}
