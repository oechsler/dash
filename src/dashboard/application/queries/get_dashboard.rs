use std::collections::HashSet;

use crate::{
    catalog::{application::ports::ApplicationReader, domain::Application},
    dashboard::{application::ports::DashboardReader, domain::UserDashboard},
    shared::application::ApplicationError,
    user::domain::{GroupId, UserId},
};

/// Read-side projection. It intentionally does not provision or mutate a dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardView {
    pub dashboard: UserDashboard,
    pub applications: Vec<Application>,
}

pub struct GetUserDashboard<D, A> {
    dashboards: D,
    applications: A,
}

impl<D: DashboardReader, A: ApplicationReader> GetUserDashboard<D, A> {
    pub fn new(dashboards: D, applications: A) -> Self {
        Self {
            dashboards,
            applications,
        }
    }

    pub async fn execute(
        &self,
        user_id: &UserId,
        groups: &HashSet<GroupId>,
    ) -> Result<DashboardView, ApplicationError> {
        let dashboard = self
            .dashboards
            .load(user_id)
            .await?
            .unwrap_or_else(|| UserDashboard::new(user_id.clone()));
        let applications = self
            .applications
            .list()
            .await?
            .into_iter()
            .filter(|application| application.is_visible_to(groups))
            .collect();
        Ok(DashboardView {
            dashboard,
            applications,
        })
    }
}
