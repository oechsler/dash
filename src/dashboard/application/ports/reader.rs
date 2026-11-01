use async_trait::async_trait;

use crate::{
    dashboard::domain::UserDashboard, shared::application::ApplicationPortError,
    shared::domain::UserId,
};

#[async_trait]
pub trait DashboardReader: Send + Sync {
    async fn load(&self, owner: &UserId) -> Result<Option<UserDashboard>, ApplicationPortError>;
}
