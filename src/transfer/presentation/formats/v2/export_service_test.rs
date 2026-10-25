use crate::{
    catalog::domain::{
        Application, ApplicationCatalogSettings, ApplicationHealthCheck, ApplicationVisibility,
        ExpectedStatusClass, HealthCheckUrl,
    },
    dashboard::domain::{Bookmark, Category, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::domain::CollectionSortOrder,
    transfer::{application::models::UserDataSnapshot, presentation::formats::TransferFormat},
    user::domain::UserId,
};

use super::V2Format;

#[test]
fn v2_supports_import_and_export_through_the_shared_format_contract() {
    let format = V2Format;
    let owner = UserId::new("user-1").unwrap();
    let payload = format
        .export(UserDataSnapshot::default(), &owner, "sam".to_owned(), false)
        .unwrap();
    let json = std::str::from_utf8(&payload).unwrap();
    assert!(json.contains("\"version\": 2"));
    assert!(json.trim_start().starts_with('{'));

    let imported = format.import(&payload, &owner).unwrap();
    assert!(imported.dashboard.is_some());
    assert!(imported.preferences.is_some());
    assert!(imported.themes.is_empty());
    assert!(imported.applications.is_empty());
}

#[test]
fn v2_round_trips_application_health_check_settings() {
    let format = V2Format;
    let owner = UserId::new("user-1").unwrap();
    let mut application = Application::create(
        "Portal",
        IconReference::parse("mdi:portal").unwrap(),
        WebAddress::parse("https://portal.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    application.set_health_check(Some(ApplicationHealthCheck::new(
        Some(HealthCheckUrl::parse("https://portal.example/health").unwrap()),
        ExpectedStatusClass::ClientError4xx,
        true,
    )));
    let mut snapshot = UserDataSnapshot::default();
    snapshot.applications.push(application);

    let payload = format
        .export(snapshot, &owner, "sam".to_owned(), true)
        .unwrap();
    let imported = format.import(&payload, &owner).unwrap();
    let health_check = imported.applications[0].health_check().unwrap();

    assert_eq!(
        health_check.url().unwrap().as_str(),
        "https://portal.example/health"
    );
    assert_eq!(
        health_check.expected_status_class(),
        ExpectedStatusClass::ClientError4xx
    );
    assert!(health_check.ignore_tls_errors());
}

#[test]
fn v2_round_trips_category_bookmarks_sort_order_and_access_time() {
    let format = V2Format;
    let owner = UserId::new("user-1").unwrap();
    let accessed_at = chrono::DateTime::parse_from_rfc3339("2026-05-02T09:30:00Z")
        .unwrap()
        .to_utc();
    let mut category = Category::new("Home lab").unwrap();
    let mut bookmark = Bookmark::new(
        "Router",
        IconReference::parse("mdi:router-network").unwrap(),
        WebAddress::parse("http://192.168.1.1").unwrap(),
    )
    .unwrap();
    category.set_bookmarks_sort_order(CollectionSortOrder::RecentlyAccessed);
    bookmark.record_access(accessed_at);
    category.add_bookmark(bookmark);
    let mut dashboard = UserDashboard::new(owner.clone());
    dashboard.add_category(category);
    dashboard.set_categories_sort_order(CollectionSortOrder::RecentlyAccessed);
    let snapshot = UserDataSnapshot {
        dashboard: Some(dashboard),
        ..UserDataSnapshot::default()
    };

    let payload = format
        .export(snapshot, &owner, "sam".to_owned(), false)
        .unwrap();
    let restored = format.import(&payload, &owner).unwrap();

    assert_eq!(
        restored.dashboard.as_ref().unwrap().categories()[0].bookmarks_sort_order(),
        CollectionSortOrder::RecentlyAccessed
    );
    assert_eq!(
        restored.dashboard.as_ref().unwrap().categories_sort_order(),
        CollectionSortOrder::RecentlyAccessed
    );
    assert_eq!(
        restored.dashboard.unwrap().categories()[0].bookmarks()[0].last_accessed_at(),
        Some(accessed_at)
    );
}

#[test]
fn admin_export_preserves_application_catalog_sort_rule_and_custom_order() {
    let format = V2Format;
    let owner = UserId::new("admin").unwrap();
    let first = Application::create(
        "Alpha",
        IconReference::parse("mdi:alpha").unwrap(),
        WebAddress::parse("https://alpha.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    let second = Application::create(
        "Beta",
        IconReference::parse("mdi:beta").unwrap(),
        WebAddress::parse("https://beta.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    let mut settings = ApplicationCatalogSettings::default();
    settings
        .set_manual_order(vec![second.id(), first.id()], &[first.id(), second.id()])
        .unwrap();
    settings.set_sort_order(CollectionSortOrder::UserDefined);
    let snapshot = UserDataSnapshot {
        applications: vec![first, second],
        application_catalog_settings: Some(settings),
        ..UserDataSnapshot::default()
    };

    let payload = format
        .export(snapshot, &owner, "admin".to_owned(), true)
        .unwrap();
    let restored = format.import(&payload, &owner).unwrap();
    let restored_settings = restored.application_catalog_settings.unwrap();

    assert_eq!(
        restored_settings.sort_order(),
        CollectionSortOrder::UserDefined
    );
    assert_eq!(restored.applications[0].name(), "Beta");
    assert_eq!(restored.applications[1].name(), "Alpha");
    assert_eq!(
        restored_settings.manual_order(),
        &[restored.applications[0].id(), restored.applications[1].id()]
    );
}
