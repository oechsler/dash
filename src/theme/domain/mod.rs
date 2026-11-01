mod color;
mod global_theme_id;
mod theme;
mod theme_appearance;
mod theme_error;
mod theme_id;
mod theme_identity;
mod theme_mode;
mod theme_palette;

#[cfg(test)]
mod color_test;
#[cfg(test)]
mod theme_test;

pub use color::{Color, ColorError, ColorResult};
pub use global_theme_id::{GlobalThemeId, GlobalThemeIdError, GlobalThemeIdResult};
pub use theme::Theme;
pub use theme_appearance::ThemeAppearance;
pub use theme_error::{DomainResult, ThemeError};
pub use theme_id::{ThemeId, ThemeIdError, ThemeIdResult};
pub use theme_identity::ThemeIdentity;
pub use theme_mode::ThemeMode;
pub use theme_palette::ThemePalette;
