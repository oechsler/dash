use crate::user::domain::UserId;

use super::{Color, GlobalThemeId, Theme, ThemeId, ThemeMode, ThemePalette, ThemeScope};

#[test]
fn global_theme_is_global_and_not_user_deletable() {
    let theme = Theme::global()
        .id(GlobalThemeId::new("catppuccin-mocha-mauve").unwrap())
        .name("Catppuccin Mocha (Mauve)")
        .mode(ThemeMode::Dark)
        .palette(ThemePalette::new(
            Color::parse("#1e1e2e").unwrap(),
            Color::parse("#cdd6f4").unwrap(),
            Color::parse("#cba6f7").unwrap(),
        ))
        .call()
        .unwrap();

    assert!(matches!(theme.id(), ThemeId::Global(_)));
    assert_eq!(theme.scope(), &ThemeScope::Global);
    assert_eq!(theme.mode(), ThemeMode::Dark);
    assert_eq!(theme.name(), "Catppuccin Mocha (Mauve)");
    assert_eq!(theme.palette().primary().as_str(), "#1e1e2e");
    assert_eq!(theme.palette().secondary().as_str(), "#cdd6f4");
    assert_eq!(theme.palette().tertiary().as_str(), "#cba6f7");
    assert!(!theme.is_deletable());
}

#[test]
fn user_defined_theme_is_owned_by_a_user_and_deletable() {
    let owner = UserId::new("user-1").unwrap();
    let palette = ThemePalette::new(
        Color::parse("#111111").unwrap(),
        Color::parse("#222222").unwrap(),
        Color::parse("#333333").unwrap(),
    );
    let theme = Theme::user_defined()
        .owner(owner.clone())
        .name("  My Theme  ")
        .mode(ThemeMode::Light)
        .palette(palette)
        .call()
        .unwrap();

    assert_eq!(theme.scope(), &ThemeScope::UserDefined(owner));
    assert_eq!(theme.mode(), ThemeMode::Light);
    assert_eq!(theme.name(), "My Theme");
    assert!(theme.is_deletable());
}
