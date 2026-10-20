use std::collections::HashMap;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{Theme, ThemeId, ThemeMode, ThemeScope},
    transfer::application::models::{UserDataSnapshot, formats::content_hash},
};

pub(super) fn merge(
    target: &mut UserDataSnapshot,
    imported: &UserDataSnapshot,
) -> Result<HashMap<ThemeId, ThemeId>, ApplicationError> {
    let mut known = HashMap::new();
    for theme in target
        .themes
        .iter()
        .filter(|theme| matches!(theme.scope(), ThemeScope::UserDefined(_)))
    {
        known.insert(theme_hash(theme), theme.id());
    }
    let mut id_map = HashMap::new();
    for theme in imported
        .themes
        .iter()
        .filter(|theme| matches!(theme.scope(), ThemeScope::UserDefined(_)))
    {
        let key = theme_hash(theme);
        let target_id = match known.get(&key) {
            Some(id) => *id,
            None => {
                target.themes.push(theme.clone());
                known.insert(key, theme.id());
                theme.id()
            }
        };
        id_map.insert(theme.id(), target_id);
    }
    Ok(id_map)
}

fn theme_hash(theme: &Theme) -> String {
    let mode = match theme.mode() {
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    content_hash(&[
        theme.name(),
        mode,
        theme.palette().primary().as_str(),
        theme.palette().secondary().as_str(),
        theme.palette().tertiary().as_str(),
    ])
}
