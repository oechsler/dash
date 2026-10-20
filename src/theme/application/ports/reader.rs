use async_trait::async_trait;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId},
    user::domain::UserId,
};

#[async_trait]
pub trait ThemeReader: Send + Sync {
    async fn list_for(&self, owner: &UserId) -> Result<Vec<Theme>, ApplicationError>;
    async fn get_for(&self, owner: &UserId, id: ThemeId)
    -> Result<Option<Theme>, ApplicationError>;
}
