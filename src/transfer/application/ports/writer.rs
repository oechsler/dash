use async_trait::async_trait;

use crate::{shared::application::ApplicationError, user::domain::UserId};

use super::snapshot::UserDataSnapshot;

#[async_trait]
pub trait UserDataWriter: Send + Sync {
    async fn save(
        &self,
        owner: &UserId,
        snapshot: &UserDataSnapshot,
    ) -> Result<(), ApplicationError>;
}
