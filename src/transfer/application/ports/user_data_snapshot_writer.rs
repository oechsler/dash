use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

#[async_trait]
pub trait UserDataSnapshotWriter: Send + Sync {
    async fn save(
        &self,
        owner: &UserId,
        snapshot: &UserDataSnapshot,
    ) -> Result<(), ApplicationError>;
}
