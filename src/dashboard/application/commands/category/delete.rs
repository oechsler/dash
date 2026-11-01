use crate::shared::application::ApplicationResult;
use crate::{
    dashboard::domain::{CategoryId, UserDashboard},
    shared::domain::UserId,
};

use crate::dashboard::application::ports::{DashboardReader, DashboardWriter};

pub struct DeleteCategory<R> {
    repository: R,
}

impl<R: DashboardReader + DashboardWriter> DeleteCategory<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, owner: &UserId, category_id: CategoryId) -> ApplicationResult<()> {
        let mut dashboard = self
            .repository
            .load(owner)
            .await?
            .unwrap_or_else(|| UserDashboard::new(owner.clone()));
        dashboard.remove_category(category_id)?;
        self.repository.save(&dashboard).await?;
        Ok(())
    }
}
