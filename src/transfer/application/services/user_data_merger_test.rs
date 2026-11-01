use super::user_data_merger::UserDataMerger;
use crate::{
    preferences::domain::UserPreferences,
    shared::domain::UserId,
    theme::domain::{Color, GlobalThemeId, Theme, ThemeMode, ThemePalette},
    transfer::application::models::{ImportedThemeNames, UserDataSnapshot},
};

fn global_dark_theme() -> Theme {
    Theme::global()
        .id(GlobalThemeId::new("destination-ocean").unwrap())
        .name("Ocean")
        .mode(ThemeMode::Dark)
        .palette(ThemePalette::new(
            Color::try_from("#111111").unwrap(),
            Color::try_from("#222222").unwrap(),
            Color::try_from("#333333").unwrap(),
        ))
        .call()
        .unwrap()
}

fn import_with_dark_theme(owner: &UserId) -> UserDataSnapshot {
    UserDataSnapshot {
        preferences: Some(UserPreferences::for_user(owner.clone())),
        imported_theme_names: Some(ImportedThemeNames {
            light: None,
            dark: Some("Ocean".to_owned()),
        }),
        ..UserDataSnapshot::default()
    }
}

#[test]
fn import_selects_matching_global_theme_by_name_and_mode() {
    let owner = UserId::try_from("user-1").unwrap();
    let theme = global_dark_theme();
    let mut target = UserDataSnapshot {
        themes: vec![theme.clone()],
        ..UserDataSnapshot::default()
    };
    let imported = import_with_dark_theme(&owner);

    UserDataMerger
        .merge(&mut target, &imported, &owner, false)
        .unwrap();

    assert_eq!(
        target.preferences.unwrap().theme_id(ThemeMode::Dark),
        Some(theme.id())
    );
}

#[test]
fn import_uses_the_default_when_a_global_theme_is_unavailable() {
    let owner = UserId::try_from("user-1").unwrap();
    let mut target = UserDataSnapshot::default();
    let imported = import_with_dark_theme(&owner);

    UserDataMerger
        .merge(&mut target, &imported, &owner, false)
        .unwrap();

    assert_eq!(target.preferences.unwrap().theme_id(ThemeMode::Dark), None);
}

#[test]
fn import_without_a_dashboard_does_not_create_one() {
    let owner = UserId::try_from("user-1").unwrap();
    let mut target = UserDataSnapshot::default();

    UserDataMerger
        .merge(&mut target, &UserDataSnapshot::default(), &owner, false)
        .unwrap();

    assert!(target.dashboard.is_none());
}
