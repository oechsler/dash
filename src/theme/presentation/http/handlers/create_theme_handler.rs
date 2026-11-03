use crate::{
    shared::{
        application::{ApplicationError, ApplicationResult},
        domain::UserId,
    },
    theme::{
        application::{
            commands::{CreateTheme, CreateThemeInput},
            ports::ThemeWriter,
        },
        domain::{ThemeId, ThemeMode},
    },
};

/// Handles `POST /api/v1/themes`.
pub async fn handle<R: ThemeWriter>(
    repository: R,
    owner: UserId,
    request: &dash_api::models::CreateThemeRequest,
) -> ApplicationResult<ThemeId> {
    let mode = match request.mode.as_str() {
        "light" => ThemeMode::Light,
        "dark" => ThemeMode::Dark,
        value => {
            return Err(ApplicationError::infrastructure(
                "decode theme mode",
                std::io::Error::new(std::io::ErrorKind::InvalidInput, value.to_owned()),
            ));
        }
    };
    CreateTheme::new(repository)
        .execute(CreateThemeInput {
            owner,
            name: request.name.clone(),
            mode,
            primary: request.palette.primary.clone(),
            secondary: request.palette.secondary.clone(),
            tertiary: request.palette.tertiary.clone(),
        })
        .await
}
