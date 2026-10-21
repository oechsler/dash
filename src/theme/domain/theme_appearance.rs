use super::{ThemeMode, ThemePalette};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeAppearance {
    Light(ThemePalette),
    Dark(ThemePalette),
}

impl ThemeAppearance {
    pub fn new(mode: ThemeMode, palette: ThemePalette) -> Self {
        match mode {
            ThemeMode::Light => Self::Light(palette),
            ThemeMode::Dark => Self::Dark(palette),
        }
    }

    pub fn mode(&self) -> ThemeMode {
        match self {
            Self::Light(_) => ThemeMode::Light,
            Self::Dark(_) => ThemeMode::Dark,
        }
    }

    pub fn palette(&self) -> &ThemePalette {
        match self {
            Self::Light(palette) | Self::Dark(palette) => palette,
        }
    }
}
