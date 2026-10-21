use crate::user::domain::UserId;

use super::{
    Color, GlobalThemeId, Theme, ThemeAppearance, ThemeId, ThemeIdentity, ThemeMode, ThemePalette,
};

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
    assert_eq!(
        theme.identity(),
        &ThemeIdentity::Global(GlobalThemeId::new("catppuccin-mocha-mauve").unwrap())
    );
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

    assert!(matches!(
        theme.identity(),
        ThemeIdentity::UserDefined { owner: theme_owner, .. } if theme_owner == &owner
    ));
    if let ThemeIdentity::UserDefined { id, .. } = theme.identity() {
        assert_eq!(theme.id(), ThemeId::UserDefined(*id));
    } else {
        unreachable!("created user theme has user-defined identity");
    }
    assert_eq!(theme.mode(), ThemeMode::Light);
    assert_eq!(theme.name(), "My Theme");
    assert!(theme.is_deletable());
}

#[test]
fn theme_appearance_keeps_mode_and_palette_together() {
    let palette = ThemePalette::new(
        Color::parse("#111111").unwrap(),
        Color::parse("#222222").unwrap(),
        Color::parse("#333333").unwrap(),
    );
    let appearance = ThemeAppearance::new(ThemeMode::Dark, palette.clone());

    assert_eq!(appearance.mode(), ThemeMode::Dark);
    assert_eq!(appearance.palette(), &palette);
    assert!(matches!(appearance, ThemeAppearance::Dark(_)));
}
