use std::collections::HashMap;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    transfer::application::models::formats::v2::ThemeExport,
    user::domain::UserId,
};

pub(super) fn build(
    owner: &UserId,
    records: Vec<ThemeExport>,
) -> Result<(Vec<Theme>, HashMap<(String, ThemeMode), ThemeId>), ApplicationError> {
    let themes = records
        .into_iter()
        .map(|record| {
            let mode = parse_mode(&record.mode)?;
            Ok(Theme::user_defined()
                .owner(owner.clone())
                .name(record.name)
                .mode(mode)
                .palette(ThemePalette::new(
                    Color::parse(record.primary)?,
                    Color::parse(record.secondary)?,
                    Color::parse(record.tertiary)?,
                ))
                .call()?)
        })
        .collect::<Result<Vec<_>, ApplicationError>>()?;
    let theme_ids = themes
        .iter()
        .map(|theme| ((theme.name().to_owned(), theme.mode()), theme.id()))
        .collect();
    Ok((themes, theme_ids))
}

fn parse_mode(value: &str) -> Result<ThemeMode, ApplicationError> {
    match value {
        "light" => Ok(ThemeMode::Light),
        "dark" => Ok(ThemeMode::Dark),
        _ => Err(
            crate::transfer::application::error::TransferError::InvalidDocument(format!(
                "unknown theme mode `{value}`"
            ))
            .into(),
        ),
    }
}
