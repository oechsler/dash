use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{preferences::domain::UserPreferences, shared::domain::UserId};

#[async_trait]
pub trait PreferencesReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> PortResult<Option<UserPreferences>>;
}
