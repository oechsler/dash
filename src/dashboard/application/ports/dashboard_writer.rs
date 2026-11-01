use crate::shared::application::PortResult;
use async_trait::async_trait;

use crate::dashboard::domain::UserDashboard;

#[async_trait]
pub trait DashboardWriter: Send + Sync {
    async fn save(&self, dashboard: &UserDashboard) -> PortResult<()>;
}
