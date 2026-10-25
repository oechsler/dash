use async_trait::async_trait;
use chrono::{TimeZone, Utc};

use crate::{
    catalog::{
        application::ports::{ApplicationCatalogSettingsReader, ApplicationReader},
        domain::{Application, ApplicationCatalogSettings, ApplicationVisibility},
    },
    dashboard::{
        application::{ports::DashboardReader, queries::GetUserDashboard},
        domain::{UserDashboard, WebAddress},
    },
    icon::domain::IconReference,
    preferences::{application::ports::PreferencesReader, domain::UserPreferences},
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    user::domain::{GroupId, UserId},
};

struct Dashboards {
    result: Option<UserDashboard>,
}

#[async_trait]
impl DashboardReader for Dashboards {
    async fn load(&self, _user_id: &UserId) -> Result<Option<UserDashboard>, ApplicationError> {
        Ok(self.result.clone())
    }
}

struct Applications(Vec<Application>);

struct CatalogSettings(Option<ApplicationCatalogSettings>);

#[async_trait]
impl ApplicationCatalogSettingsReader for CatalogSettings {
    async fn load(&self) -> Result<Option<ApplicationCatalogSettings>, ApplicationError> {
        Ok(self.0.clone())
    }
}

struct Preferences(Option<UserPreferences>);

#[async_trait]
impl PreferencesReader for Preferences {
    async fn load(&self, _owner: &UserId) -> Result<Option<UserPreferences>, ApplicationError> {
        Ok(self.0.clone())
    }
}

#[async_trait]
impl ApplicationReader for Applications {
    async fn list(&self) -> Result<Vec<Application>, ApplicationError> {
        Ok(self.0.clone())
    }

    async fn get(
        &self,
        id: crate::catalog::domain::ApplicationId,
    ) -> Result<Option<Application>, ApplicationError> {
        Ok(self
            .0
            .iter()
            .find(|application| application.id() == id)
            .cloned())
    }
}

fn app(name: &str, visibility: ApplicationVisibility) -> Application {
    Application::builder()
        .name(name)
        .icon(IconReference::parse("mdi:home").unwrap())
        .address(WebAddress::parse("https://example.test").unwrap())
        .visibility(visibility)
        .build()
        .unwrap()
}

#[tokio::test]
async fn query_returns_empty_dashboard_and_filters_applications() {
    let group = GroupId::new("staff").unwrap();
    let dashboards = Dashboards { result: None };
    let query = GetUserDashboard::new(
        dashboards,
        Applications(vec![
            app("Public", ApplicationVisibility::everyone()),
            app("Staff", ApplicationVisibility::groups([group.clone()])),
            app(
                "Admins",
                ApplicationVisibility::groups([GroupId::new("admins").unwrap()]),
            ),
        ]),
        CatalogSettings(None),
        Preferences(None),
    );

    let view = query
        .execute(
            &UserId::new("user-1").unwrap(),
            &std::collections::HashSet::from([group]),
        )
        .await
        .unwrap();

    assert!(view.categories.is_empty());
    assert!(view.shelved_categories.is_empty());
    assert_eq!(view.applications.len(), 2);
    assert_eq!(view.applications[0].name(), "Public");
    assert_eq!(view.applications[1].name(), "Staff");
}

#[tokio::test]
async fn query_sorts_bookmarks_alphabetically_inside_each_category() {
    let owner = UserId::new("user-1").unwrap();
    let mut dashboard = UserDashboard::new(owner.clone());
    let mut category = crate::dashboard::domain::Category::new("Home lab").unwrap();
    category.set_bookmarks_sort_order(CollectionSortOrder::Alphabetical);
    category.add_bookmark(
        crate::dashboard::domain::Bookmark::new(
            "Zebra",
            IconReference::parse("mdi:link").unwrap(),
            WebAddress::parse("https://zebra.example").unwrap(),
        )
        .unwrap(),
    );
    category.add_bookmark(
        crate::dashboard::domain::Bookmark::new(
            "Alpha",
            IconReference::parse("mdi:link").unwrap(),
            WebAddress::parse("https://alpha.example").unwrap(),
        )
        .unwrap(),
    );
    dashboard.add_category(category);
    let query = GetUserDashboard::new(
        Dashboards {
            result: Some(dashboard),
        },
        Applications(Vec::new()),
        CatalogSettings(None),
        Preferences(None),
    );

    let view = query
        .execute(&owner, &std::collections::HashSet::new())
        .await
        .unwrap();

    let bookmarks = view.categories[0].bookmarks();
    assert_eq!(bookmarks[0].name(), "Alpha");
    assert_eq!(bookmarks[1].name(), "Zebra");
}

#[tokio::test]
async fn query_sorts_bookmarks_by_recent_access_inside_each_category() {
    let owner = UserId::new("user-1").unwrap();
    let mut dashboard = UserDashboard::new(owner.clone());
    let mut category = crate::dashboard::domain::Category::new("Home lab").unwrap();
    category.set_bookmarks_sort_order(CollectionSortOrder::RecentlyAccessed);
    let mut old_bookmark = crate::dashboard::domain::Bookmark::new(
        "Old link",
        IconReference::parse("mdi:link").unwrap(),
        WebAddress::parse("https://old.example").unwrap(),
    )
    .unwrap();
    old_bookmark.record_access(Utc.with_ymd_and_hms(2026, 5, 1, 9, 0, 0).unwrap());
    let mut recent_bookmark = crate::dashboard::domain::Bookmark::new(
        "Recent link",
        IconReference::parse("mdi:link").unwrap(),
        WebAddress::parse("https://recent.example").unwrap(),
    )
    .unwrap();
    recent_bookmark.record_access(Utc.with_ymd_and_hms(2026, 5, 2, 9, 0, 0).unwrap());
    category.add_bookmark(old_bookmark);
    category.add_bookmark(recent_bookmark);
    dashboard.add_category(category);
    let query = GetUserDashboard::new(
        Dashboards {
            result: Some(dashboard),
        },
        Applications(Vec::new()),
        CatalogSettings(None),
        Preferences(None),
    );

    let view = query
        .execute(&owner, &std::collections::HashSet::new())
        .await
        .unwrap();

    let bookmarks = view.categories[0].bookmarks();
    assert_eq!(bookmarks[0].name(), "Recent link");
    assert_eq!(bookmarks[1].name(), "Old link");
}

#[tokio::test]
async fn query_sorts_visible_apps_by_the_current_users_recent_access() {
    let owner = UserId::new("user-1").unwrap();
    let alpha = app("Alpha", ApplicationVisibility::everyone());
    let beta = app("Beta", ApplicationVisibility::everyone());
    let mut settings = ApplicationCatalogSettings::default();
    settings.set_sort_order(CollectionSortOrder::RecentlyAccessed);
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.record_application_access(beta.id());
    let query = GetUserDashboard::new(
        Dashboards { result: None },
        Applications(vec![alpha, beta]),
        CatalogSettings(Some(settings)),
        Preferences(Some(preferences)),
    );

    let view = query
        .execute(&owner, &std::collections::HashSet::new())
        .await
        .unwrap();

    assert_eq!(view.applications[0].name(), "Beta");
    assert_eq!(view.applications[1].name(), "Alpha");
}

#[tokio::test]
async fn query_sorts_all_categories_before_splitting_shelved_categories() {
    let owner = UserId::new("user-1").unwrap();
    let mut dashboard = UserDashboard::new(owner.clone());
    dashboard.add_category(crate::dashboard::domain::Category::new("Zulu").unwrap());
    dashboard.add_category(crate::dashboard::domain::ShelvedCategory::from(
        crate::dashboard::domain::Category::new("Alpha").unwrap(),
    ));
    dashboard.add_category(crate::dashboard::domain::Category::new("Beta").unwrap());
    dashboard.set_categories_sort_order(CollectionSortOrder::Alphabetical);
    let query = GetUserDashboard::new(
        Dashboards {
            result: Some(dashboard),
        },
        Applications(Vec::new()),
        CatalogSettings(None),
        Preferences(None),
    );

    let view = query
        .execute(&owner, &std::collections::HashSet::new())
        .await
        .unwrap();

    assert_eq!(
        view.categories_sort_order,
        CollectionSortOrder::Alphabetical
    );
    assert_eq!(view.categories[0].name(), "Beta");
    assert_eq!(view.categories[1].name(), "Zulu");
    assert_eq!(view.shelved_categories[0].name(), "Alpha");
}
