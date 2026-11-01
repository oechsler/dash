use crate::shared::domain::UserId;

use crate::theme::domain::{Color, GlobalThemeId, Theme, ThemeMode, ThemePalette};

use super::{Language, PreferencesError, TimeZone, TimeZoneError, UserPreferences};

fn palette() -> ThemePalette {
    ThemePalette::new(
        Color::try_from("#111111").unwrap(),
        Color::try_from("#222222").unwrap(),
        Color::try_from("#333333").unwrap(),
    )
}

#[test]
fn preferences_default_to_no_theme_and_automatic_language_and_timezone() {
    let preferences = UserPreferences::for_user(UserId::try_from("user-1").unwrap());

    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert_eq!(preferences.theme_id(ThemeMode::Dark), None);
    assert_eq!(preferences.language(), Language::Auto);
    assert_eq!(preferences.time_zone(), &TimeZone::Auto);
}

#[test]
fn recently_accessed_applications_move_to_the_front_without_duplicates() {
    let user_id = UserId::try_from("user-1").unwrap();
    let mut preferences = UserPreferences::for_user(user_id);
    let first = crate::catalog::domain::ApplicationId::new();
    let second = crate::catalog::domain::ApplicationId::new();

    preferences.record_application_access(first);
    preferences.record_application_access(second);
    preferences.record_application_access(first);

    assert_eq!(
        preferences.recently_accessed_application_ids(),
        &[first, second]
    );
}

#[test]
fn preferences_store_selected_theme_ids_per_mode() {
    let owner = UserId::try_from("user-1").unwrap();
    let own_theme = Theme::user_defined()
        .owner(owner.clone())
        .name("My theme")
        .mode(ThemeMode::Dark)
        .palette(palette())
        .call()
        .unwrap();
    let mut preferences = UserPreferences::for_user(owner.clone());

    preferences.select_theme(own_theme.mode(), own_theme.id());
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert!(preferences.uses_theme(own_theme.id()));
    let global = Theme::global()
        .id(GlobalThemeId::new("global-light").unwrap())
        .name("Global light")
        .mode(ThemeMode::Light)
        .palette(palette())
        .call()
        .unwrap();
    preferences.select_theme(global.mode(), global.id());
    assert_eq!(preferences.theme_id(ThemeMode::Light), Some(global.id()));
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
    assert!(preferences.uses_theme(own_theme.id()));

    preferences.clear_theme_selection(ThemeMode::Light);
    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
}

#[test]
fn selected_theme_is_detected_until_deselected_from_its_mode() {
    let owner = UserId::try_from("user-1").unwrap();
    let theme = Theme::user_defined()
        .owner(owner.clone())
        .name("Personal dark")
        .mode(ThemeMode::Dark)
        .palette(palette())
        .call()
        .unwrap();
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.select_theme(theme.mode(), theme.id());

    assert_eq!(
        preferences.ensure_theme_is_not_selected(theme.id()),
        Err(PreferencesError::ThemeCurrentlySelected {
            theme_id: theme.id(),
            user_id: owner,
        })
    );

    preferences.clear_theme_selection(ThemeMode::Dark);
    assert_eq!(preferences.ensure_theme_is_not_selected(theme.id()), Ok(()));
}

#[test]
fn language_and_timezone_preferences_are_typed_and_validated() {
    let mut preferences = UserPreferences::for_user(UserId::try_from("user-1").unwrap());
    preferences.set_language(Language::German);
    preferences.set_time_zone(TimeZone::try_from("Europe/Vienna").unwrap());

    assert_eq!(preferences.language(), Language::German);
    assert_eq!(preferences.time_zone().as_str(), "Europe/Vienna");
    assert_eq!(TimeZone::try_from("auto").unwrap(), TimeZone::Auto);
    assert!(matches!(
        TimeZone::try_from("Not/ATimezone"),
        Err(PreferencesError::TimeZone(
            TimeZoneError::InvalidIanaName { .. }
        ))
    ));
}
