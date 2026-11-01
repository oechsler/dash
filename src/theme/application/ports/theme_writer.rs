use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{
    shared::domain::UserId,
    theme::domain::{Theme, ThemeId},
};

#[async_trait]
pub trait ThemeWriter: Send + Sync {
    async fn save(&self, theme: &Theme) -> PortResult<()>;
    async fn delete(&self, owner: &UserId, id: ThemeId) -> PortResult<()>;
}
