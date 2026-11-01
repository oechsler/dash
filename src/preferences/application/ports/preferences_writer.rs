use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::preferences::domain::UserPreferences;

#[async_trait]
pub trait PreferencesWriter: Send + Sync {
    async fn save(&self, preferences: &UserPreferences) -> PortResult<()>;
}
