use crate::user::domain::UserId;

use crate::theme::domain::{Color, GlobalThemeId, Theme, ThemeMode, ThemePalette};

use super::{Language, PreferencesError, TimeZone, TimeZoneError, UserPreferences};

fn palette() -> ThemePalette {
    ThemePalette::new(
        Color::parse("#111111").unwrap(),
        Color::parse("#222222").unwrap(),
        Color::parse("#333333").unwrap(),
    )
}

#[test]
fn preferences_default_to_no_theme_and_automatic_language_and_timezone() {
    let preferences = UserPreferences::for_user(UserId::new("user-1").unwrap());

    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert_eq!(preferences.theme_id(ThemeMode::Dark), None);
    assert_eq!(preferences.language(), Language::Auto);
    assert_eq!(preferences.time_zone(), &TimeZone::Auto);
}

#[test]
fn preferences_can_select_only_their_owners_custom_theme() {
    let owner = UserId::new("user-1").unwrap();
    let other = UserId::new("user-2").unwrap();
    let own_theme = Theme::user_defined()
        .owner(owner.clone())
        .name("My theme")
        .mode(ThemeMode::Dark)
        .palette(palette())
        .call()
        .unwrap();
    let other_theme = Theme::user_defined()
        .owner(other)
        .name("Other theme")
        .mode(ThemeMode::Dark)
        .palette(palette())
        .call()
        .unwrap();
    let mut preferences = UserPreferences::for_user(owner.clone());

    preferences.select_theme(&own_theme).unwrap();
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert!(preferences.uses_theme(own_theme.id()));
    assert_eq!(
        preferences.select_theme(&other_theme),
        Err(PreferencesError::ThemeNotAvailableToUser {
            theme_id: other_theme.id(),
            user_id: owner,
        })
    );

    let global = Theme::global()
        .id(GlobalThemeId::new("global-light").unwrap())
        .name("Global light")
        .mode(ThemeMode::Light)
        .palette(palette())
        .call()
        .unwrap();
    preferences.select_theme(&global).unwrap();
    assert_eq!(preferences.theme_id(ThemeMode::Light), Some(global.id()));
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
    assert!(preferences.uses_theme(own_theme.id()));

    preferences.clear_theme_selection(ThemeMode::Light);
    assert_eq!(preferences.theme_id(ThemeMode::Light), None);
    assert_eq!(preferences.theme_id(ThemeMode::Dark), Some(own_theme.id()));
}

#[test]
fn selected_theme_cannot_be_deleted_until_deselected_from_its_mode() {
    let owner = UserId::new("user-1").unwrap();
    let theme = Theme::user_defined()
        .owner(owner.clone())
        .name("Personal dark")
        .mode(ThemeMode::Dark)
        .palette(palette())
        .call()
        .unwrap();
    let mut preferences = UserPreferences::for_user(owner.clone());
    preferences.select_theme(&theme).unwrap();

    assert_eq!(
        preferences.ensure_theme_can_be_deleted(&theme),
        Err(PreferencesError::ThemeCurrentlySelected {
            theme_id: theme.id(),
            user_id: owner,
        })
    );

    preferences.clear_theme_selection(ThemeMode::Dark);
    assert_eq!(preferences.ensure_theme_can_be_deleted(&theme), Ok(()));
}

#[test]
fn global_theme_cannot_be_deleted() {
    let preferences = UserPreferences::for_user(UserId::new("user-1").unwrap());
    let theme = Theme::global()
        .id(GlobalThemeId::new("brand-default").unwrap())
        .name("Brand default")
        .mode(ThemeMode::Light)
        .palette(palette())
        .call()
        .unwrap();

    assert_eq!(
        preferences.ensure_theme_can_be_deleted(&theme),
        Err(PreferencesError::ThemeNotDeletable {
            theme_id: theme.id()
        })
    );
}

#[test]
fn language_and_timezone_preferences_are_typed_and_validated() {
    let mut preferences = UserPreferences::for_user(UserId::new("user-1").unwrap());
    preferences.set_language(Language::German);
    preferences.set_time_zone(TimeZone::parse("Europe/Vienna").unwrap());

    assert_eq!(preferences.language(), Language::German);
    assert_eq!(preferences.time_zone().as_str(), "Europe/Vienna");
    assert_eq!(TimeZone::parse("auto").unwrap(), TimeZone::Auto);
    assert!(matches!(
        TimeZone::parse("Not/ATimezone"),
        Err(PreferencesError::TimeZone(
            TimeZoneError::InvalidIanaName { .. }
        ))
    ));
}
