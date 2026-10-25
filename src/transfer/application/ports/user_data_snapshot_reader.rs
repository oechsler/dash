use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

#[async_trait]
pub trait UserDataSnapshotReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> Result<UserDataSnapshot, ApplicationError>;
}
