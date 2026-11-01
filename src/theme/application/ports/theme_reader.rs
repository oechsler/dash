use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{
    shared::domain::UserId,
    theme::domain::{Theme, ThemeId},
};

#[async_trait]
pub trait ThemeReader: Send + Sync {
    async fn list_for(&self, owner: &UserId) -> PortResult<Vec<Theme>>;
    async fn get_for(&self, owner: &UserId, id: ThemeId) -> PortResult<Option<Theme>>;
}
