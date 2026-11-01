use async_trait::async_trait;

use crate::{preferences::domain::UserPreferences, shared::application::ApplicationPortError};

#[async_trait]
pub trait PreferencesWriter: Send + Sync {
    async fn save(&self, preferences: &UserPreferences) -> Result<(), ApplicationPortError>;
}
