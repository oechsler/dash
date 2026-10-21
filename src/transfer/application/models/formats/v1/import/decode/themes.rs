use std::collections::HashMap;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
    user::domain::UserId,
};

use super::super::model::GoThemeV1;

pub(super) fn build(
    owner: &UserId,
    records: Vec<GoThemeV1>,
) -> Result<(Vec<Theme>, HashMap<String, ThemeId>), ApplicationError> {
    let themes = records
        .into_iter()
        .map(|record| {
            Ok(Theme::user_defined()
                .owner(owner.clone())
                .name(record.name)
                .mode(ThemeMode::Light)
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
        .map(|theme| (theme.name().to_owned(), theme.id()))
        .collect();
    Ok((themes, theme_ids))
}
