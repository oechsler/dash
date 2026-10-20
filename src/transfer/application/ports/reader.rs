use async_trait::async_trait;

use crate::{shared::application::ApplicationError, user::domain::UserId};

use super::snapshot::UserDataSnapshot;

#[async_trait]
pub trait UserDataReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> Result<UserDataSnapshot, ApplicationError>;
}
