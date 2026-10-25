use std::collections::HashSet;

use crate::{
    catalog::{
        application::ports::{ApplicationCatalogSettingsReader, ApplicationReader},
        domain::{Application, ApplicationCatalogSettings},
    },
    dashboard::{
        application::ports::DashboardReader,
        domain::{Category, ShelvedCategory, UserDashboard},
    },
    preferences::{application::ports::PreferencesReader, domain::UserPreferences},
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    user::domain::{GroupId, UserId},
};

/// Read-side projection. It intentionally does not provision or mutate a dashboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardView {
    pub categories_sort_order: CollectionSortOrder,
    pub categories: Vec<Category>,
    pub shelved_categories: Vec<ShelvedCategory>,
    pub applications: Vec<Application>,
}

pub struct GetUserDashboard<D, A, S, P> {
    dashboards: D,
    applications: A,
    catalog_settings: S,
    preferences: P,
}

impl<D, A, S, P> GetUserDashboard<D, A, S, P>
where
    D: DashboardReader,
    A: ApplicationReader,
    S: ApplicationCatalogSettingsReader,
    P: PreferencesReader,
{
    pub fn new(dashboards: D, applications: A, catalog_settings: S, preferences: P) -> Self {
        Self {
            dashboards,
            applications,
            catalog_settings,
            preferences,
        }
    }

    pub async fn execute(
        &self,
        user_id: &UserId,
        groups: &HashSet<GroupId>,
    ) -> Result<DashboardView, ApplicationError> {
        let mut dashboard = self
            .dashboards
            .load(user_id)
            .await?
            .unwrap_or_else(|| UserDashboard::new(user_id.clone()));
        let settings = self
            .catalog_settings
            .load()
            .await?
            .unwrap_or_else(ApplicationCatalogSettings::default);
        let preferences = self
            .preferences
            .load(user_id)
            .await?
            .unwrap_or_else(|| UserPreferences::for_user(user_id.clone()));
        let categories_sort_order = dashboard.categories_sort_order();
        dashboard.sort_categories_for_display();
        dashboard.sort_bookmarks_for_display();
        let mut applications: Vec<_> = self
            .applications
            .list()
            .await?
            .into_iter()
            .filter(|application| application.is_visible_to(groups))
            .collect();
        settings.sort_applications(
            &mut applications,
            preferences.recently_accessed_application_ids(),
        );
        let (categories, shelved_categories) =
            dashboard
                .categories()
                .iter()
                .fold((Vec::new(), Vec::new()), |mut result, category| {
                    if let Some(category) = category.as_active() {
                        result.0.push(category.clone());
                    } else if let Some(category) = category.as_shelved() {
                        result.1.push(category.clone());
                    }
                    result
                });
        Ok(DashboardView {
            categories_sort_order,
            categories,
            shelved_categories,
            applications,
        })
    }
}
