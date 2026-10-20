use std::collections::HashSet;

use crate::{dashboard::domain::WebAddress, icon::domain::IconReference, user::domain::GroupId};

use super::{Application, ApplicationVisibility};

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
