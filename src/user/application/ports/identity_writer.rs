use async_trait::async_trait;

use crate::{shared::application::ApplicationError, user::domain::UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedUser {
    pub user_id: UserId,
    pub is_new: bool,
}

#[async_trait]
pub trait IdentityWriter: Send + Sync {
    async fn resolve_or_create(
        &self,
        issuer: &str,
        subject: &str,
    ) -> Result<ResolvedUser, ApplicationError>;
}
