use async_trait::async_trait;

use crate::{
    preferences::domain::UserPreferences, shared::application::ApplicationError,
    user::domain::UserId,
};

#[async_trait]
pub trait PreferencesReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> Result<Option<UserPreferences>, ApplicationError>;
}
