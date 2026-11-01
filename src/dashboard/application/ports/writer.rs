use async_trait::async_trait;

use crate::{dashboard::domain::UserDashboard, shared::application::ApplicationPortError};

#[async_trait]
pub trait DashboardWriter: Send + Sync {
    async fn save(&self, dashboard: &UserDashboard) -> Result<(), ApplicationPortError>;
}
