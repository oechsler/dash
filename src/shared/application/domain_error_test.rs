use std::error::Error;

use crate::{
    catalog::domain::{Application, ApplicationVisibility, CatalogApplicationError, CatalogError},
    dashboard::domain::{Category, DashboardError, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationDomainError,
    shared::domain::UserId,
};

#[test]
fn errors_keep_context_from_the_domain_down_to_the_entity() {
    let mut dashboard = UserDashboard::new(UserId::try_from("user-1").unwrap());
    let category = Category::new("Work").unwrap();
    let category_id = category.id();
    dashboard.add_category(category);

    let dashboard_error = dashboard.rename_category(category_id, " ").unwrap_err();
    let domain_error = ApplicationDomainError::from(dashboard_error);

    assert_eq!(
        domain_error.to_string(),
        "dashboard domain error: dashboard category operation failed: category name must not be empty (received \" \")"
    );

    let dashboard_error = domain_error.source().unwrap();
    assert!(matches!(
        dashboard_error.downcast_ref::<DashboardError>(),
        Some(DashboardError::Category(_))
    ));
    assert!(dashboard_error.source().is_some());
}

#[test]
fn application_errors_flow_through_catalog_to_domain_error() {
    let catalog_error = Application::builder()
        .name(" ")
        .icon(IconReference::try_from("pack:app").unwrap())
        .address(WebAddress::try_from("https://app.example").unwrap())
        .visibility(ApplicationVisibility::everyone())
        .build()
        .unwrap_err();
    let domain_error = ApplicationDomainError::from(catalog_error);

    assert!(matches!(
        domain_error,
        ApplicationDomainError::Catalog(CatalogError::Application(
            CatalogApplicationError::EmptyName { .. }
        ))
    ));
    let catalog_error = domain_error.source().unwrap();
    assert!(matches!(
        catalog_error.downcast_ref::<CatalogError>(),
        Some(CatalogError::Application(_))
    ));
    assert!(catalog_error.source().is_some());
}
