use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::{dashboard::domain::UserDashboard, shared::domain::UserId};

#[async_trait]
pub trait DashboardReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> PortResult<Option<UserDashboard>>;
}
