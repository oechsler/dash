use async_trait::async_trait;

use crate::{dashboard::domain::UserDashboard, shared::application::ApplicationError};

#[async_trait]
pub trait DashboardWriter: Send + Sync {
    async fn save(&self, dashboard: &UserDashboard) -> Result<(), ApplicationError>;
}
