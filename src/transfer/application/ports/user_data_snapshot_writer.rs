use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{shared::domain::UserId, transfer::application::models::UserDataSnapshot};

#[async_trait]
pub trait UserDataSnapshotWriter: Send + Sync {
    async fn save(&self, owner: &UserId, snapshot: &UserDataSnapshot) -> PortResult<()>;
}
