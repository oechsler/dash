use async_trait::async_trait;

use crate::{
    shared::application::ApplicationPortError, shared::domain::UserId,
    transfer::application::models::UserDataSnapshot,
};

#[async_trait]
pub trait UserDataSnapshotReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> Result<UserDataSnapshot, ApplicationPortError>;
}
