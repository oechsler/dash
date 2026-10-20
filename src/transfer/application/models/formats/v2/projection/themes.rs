use crate::{
    theme::domain::{Theme, ThemeId, ThemeMode},
    transfer::application::models::formats::{content_hash, v2::ThemeExport},
};

pub(super) fn name_for(themes: &[Theme], id: ThemeId) -> Option<String> {
    themes
        .iter()
        .find(|theme| theme.id() == id)
        .map(|theme| theme.name().to_owned())
}

pub(super) fn export(theme: &Theme) -> ThemeExport {
    let mode = match theme.mode() {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    }
    .to_owned();
    let name = theme.name().to_owned();
    let primary = theme.palette().primary().as_str().to_owned();
    let secondary = theme.palette().secondary().as_str().to_owned();
    let tertiary = theme.palette().tertiary().as_str().to_owned();
    let hash = content_hash(&[&name, &mode, &primary, &secondary, &tertiary]);
    ThemeExport {
        hash,
        name,
        mode,
        primary,
        secondary,
        tertiary,
    }
}
