use crate::{
    shared::application::ApplicationError,
    theme::{
        application::ports::ThemeWriter,
        domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    },
    user::domain::UserId,
};

pub struct CreateThemeInput {
    pub owner: UserId,
    pub name: String,
    pub mode: ThemeMode,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

pub struct CreateTheme<R> {
    repository: R,
}

impl<R: ThemeWriter> CreateTheme<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: CreateThemeInput) -> Result<ThemeId, ApplicationError> {
        let palette = ThemePalette::new(
            Color::parse(input.primary)?,
            Color::parse(input.secondary)?,
            Color::parse(input.tertiary)?,
        );
        let theme = Theme::user_defined()
            .owner(input.owner)
            .name(input.name)
            .mode(input.mode)
            .palette(palette)
            .call()?;
        let id = theme.id();
        self.repository.save(&theme).await?;
        Ok(id)
    }
}
