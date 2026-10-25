use std::collections::HashSet;

use crate::{dashboard::domain::WebAddress, icon::domain::IconReference, user::domain::GroupId};

use super::{
    Application, ApplicationHealth, ApplicationHealthCheck, ApplicationVisibility,
    ExpectedStatusClass, HealthCheckUrl,
};

fn group(id: &str) -> GroupId {
    GroupId::new(id).unwrap()
}

fn icon(value: &str) -> IconReference {
    IconReference::parse(value).unwrap()
}

#[test]
fn visibility_uses_any_matching_group_and_no_groups_means_everyone() {
    let user_groups = HashSet::from([group("engineering")]);
    let public = Application::builder()
        .name("Public")
        .icon(icon("mdi:public"))
        .address(WebAddress::parse("https://public.example").unwrap())
        .visibility(ApplicationVisibility::everyone())
        .build()
        .unwrap();
    let restricted = Application::builder()
        .name("Engineering")
        .icon(icon("custom-pack:engineering"))
        .address(WebAddress::parse("https://engineering.example").unwrap())
        .visibility(ApplicationVisibility::groups([
            group("engineering"),
            group("operations"),
        ]))
        .build()
        .unwrap();
    let hidden = Application::builder()
        .name("Admin")
        .icon(icon("https://assets.example/admin.svg"))
        .address(WebAddress::parse("https://admin.example").unwrap())
        .visibility(ApplicationVisibility::groups([group("administrators")]))
        .build()
        .unwrap();

    assert!(public.is_visible_to(&HashSet::new()));
    assert!(restricted.is_visible_to(&user_groups));
    assert!(!hidden.is_visible_to(&user_groups));
}

#[test]
fn application_can_be_edited_without_bypassing_name_validation() {
    let mut application = Application::builder()
        .name("Portal")
        .icon(icon("mdi:portal"))
        .address(WebAddress::parse("https://portal.example").unwrap())
        .visibility(ApplicationVisibility::everyone())
        .build()
        .unwrap();

    application.rename("  Team portal ").unwrap();
    application.change_icon(icon("brand:portal"));
    application.change_address(WebAddress::parse("https://team.example").unwrap());
    application.restrict_to([group("engineering")]);

    assert_eq!(application.name(), "Team portal");
    assert_eq!(application.icon().as_str(), "brand:portal");
    assert_eq!(application.address().as_str(), "https://team.example");
    assert!(!application.is_visible_to(&HashSet::new()));
}

#[test]
fn health_check_is_optional_and_defaults_to_the_application_address() {
    let mut application = Application::create(
        "Portal",
        icon("mdi:portal"),
        WebAddress::parse("https://portal.example/health").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();

    assert!(application.health_check().is_none());
    assert_eq!(
        application.health_check_target(),
        None,
        "an app without a health-check configuration is not polled"
    );

    application.set_health_check(Some(ApplicationHealthCheck::new(
        None,
        ExpectedStatusClass::Success2xx,
        false,
    )));

    assert_eq!(
        application.health_check_target(),
        Some("https://portal.example/health")
    );
}

#[test]
fn health_check_can_use_a_custom_url_status_class_and_tls_override() {
    let health_check = ApplicationHealthCheck::new(
        Some(HealthCheckUrl::parse("https://portal.example/status").unwrap()),
        ExpectedStatusClass::ClientError4xx,
        true,
    );

    assert_eq!(
        health_check.expected_status_class(),
        ExpectedStatusClass::ClientError4xx
    );
    assert!(health_check.ignore_tls_errors());
    assert_eq!(
        health_check.url().unwrap().as_str(),
        "https://portal.example/status"
    );
    assert!(ExpectedStatusClass::ClientError4xx.accepts(401));
    assert!(!ExpectedStatusClass::ClientError4xx.accepts(503));
}

#[test]
fn health_check_url_only_accepts_http_or_https() {
    assert!(HealthCheckUrl::parse("https://portal.example/health").is_ok());
    assert!(HealthCheckUrl::parse("http://portal.example/health").is_ok());
    assert!(HealthCheckUrl::parse("ftp://portal.example/health").is_err());
}

#[test]
fn health_check_result_uses_the_configured_status_class() {
    let mut application = Application::create(
        "Portal",
        icon("mdi:portal"),
        WebAddress::parse("https://portal.example/health").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    application.set_health_check(Some(ApplicationHealthCheck::new(
        None,
        ExpectedStatusClass::ClientError4xx,
        false,
    )));
    application.record_health_check(Some(401));
    assert_eq!(application.health(), Some(ApplicationHealth::Up));

    application.record_health_check(None);
    assert_eq!(application.health(), Some(ApplicationHealth::Down));
}
