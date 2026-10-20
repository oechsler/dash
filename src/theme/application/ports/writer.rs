use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId},
    user::domain::UserId,
};

#[async_trait]
pub trait ThemeWriter: Send + Sync {
    async fn save(&self, theme: &Theme) -> Result<(), ApplicationError>;
    async fn delete(&self, owner: &UserId, id: ThemeId) -> Result<(), ApplicationError>;
}
