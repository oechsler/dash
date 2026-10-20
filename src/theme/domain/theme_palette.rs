use super::Color;
use getset::Getters;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ThemePalette {
    #[getset(get = "pub")]
    primary: Color,
    #[getset(get = "pub")]
    secondary: Color,
    #[getset(get = "pub")]
    tertiary: Color,
}

impl ThemePalette {
    pub fn new(primary: Color, secondary: Color, tertiary: Color) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
        }
    }
}
