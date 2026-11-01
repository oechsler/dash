use async_trait::async_trait;

use crate::{
    shared::application::ApplicationPortError,
    shared::domain::UserId,
    theme::domain::{Theme, ThemeId},
};

#[async_trait]
pub trait ThemeWriter: Send + Sync {
    async fn save(&self, theme: &Theme) -> Result<(), ApplicationPortError>;
    async fn delete(&self, owner: &UserId, id: ThemeId) -> Result<(), ApplicationPortError>;
}
