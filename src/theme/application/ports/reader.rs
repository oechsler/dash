use async_trait::async_trait;

use crate::{
    shared::application::ApplicationPortError,
    shared::domain::UserId,
    theme::domain::{Theme, ThemeId},
};

#[async_trait]
pub trait ThemeReader: Send + Sync {
    async fn list_for(&self, owner: &UserId) -> Result<Vec<Theme>, ApplicationPortError>;
    async fn get_for(
        &self,
        owner: &UserId,
        id: ThemeId,
    ) -> Result<Option<Theme>, ApplicationPortError>;
}
