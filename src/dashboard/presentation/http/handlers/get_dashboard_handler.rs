use std::collections::HashSet;

use crate::{
    catalog::application::ports::{ApplicationCatalogSettingsReader, ApplicationReader},
    dashboard::application::{
        ports::DashboardReader,
        queries::{DashboardView, GetUserDashboard},
    },
    preferences::application::ports::PreferencesReader,
    shared::{
        application::ApplicationResult,
        domain::{GroupId, UserId},
    },
};

/// Handles `GET /api/v1/dashboard`.
pub async fn handle<D, A, S, P>(
    dashboards: D,
    applications: A,
    catalog_settings: S,
    preferences: P,
    owner: &UserId,
    groups: &HashSet<GroupId>,
) -> ApplicationResult<DashboardView>
where
    D: DashboardReader,
    A: ApplicationReader,
    S: ApplicationCatalogSettingsReader,
    P: PreferencesReader,
{
    GetUserDashboard::new(dashboards, applications, catalog_settings, preferences)
        .execute(owner, groups)
        .await
}
