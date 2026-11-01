use crate::{
    catalog::domain::{
        Application, ApplicationCatalogSettings, ApplicationId, ApplicationVisibility,
    },
    icon::domain::IconReference,
    shared::domain::CollectionSortOrder,
    shared::domain::WebAddress,
};

fn application(name: &str) -> Application {
    Application::create(
        name,
        IconReference::try_from("mdi:home").unwrap(),
        WebAddress::try_from("https://example.test").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap()
}

#[test]
fn application_catalog_can_sort_by_name_recent_access_and_manual_order() {
    let zebra = application("Zebra");
    let alpha = application("Alpha");
    let apps = vec![zebra.clone(), alpha.clone()];
    let mut settings = ApplicationCatalogSettings::default();
    let mut ordered = apps.clone();

    settings.set_sort_order(CollectionSortOrder::Alphabetical);
    settings.sort_applications(&mut ordered, &[]);
    assert_eq!(ordered[0].name(), "Alpha");

    settings.set_sort_order(CollectionSortOrder::RecentlyAccessed);
    settings.sort_applications(&mut ordered, &[zebra.id(), alpha.id()]);
    assert_eq!(ordered[0].name(), "Zebra");

    settings
        .set_manual_order(vec![zebra.id(), alpha.id()], &[zebra.id(), alpha.id()])
        .unwrap();
    settings.set_sort_order(CollectionSortOrder::UserDefined);
    settings.sort_applications(&mut ordered, &[]);
    assert_eq!(ordered[0].name(), "Zebra");
}

#[test]
fn manual_application_order_must_contain_each_id_exactly_once() {
    let first = ApplicationId::new();
    let second = ApplicationId::new();
    let mut settings = ApplicationCatalogSettings::default();

    assert!(
        settings
            .set_manual_order(vec![first, first], &[first, second])
            .is_err()
    );
    assert!(
        settings
            .set_manual_order(vec![first], &[first, second])
            .is_err()
    );
}
