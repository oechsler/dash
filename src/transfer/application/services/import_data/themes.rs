use std::collections::HashMap;

use crate::{
    shared::application::ApplicationError,
    theme::domain::{ThemeId, ThemeIdentity},
    transfer::application::models::{UserDataSnapshot, formats::theme_hash},
};

pub(super) fn merge(
    target: &mut UserDataSnapshot,
    imported: &UserDataSnapshot,
) -> Result<HashMap<ThemeId, ThemeId>, ApplicationError> {
    let mut known = HashMap::new();
    for theme in target
        .themes
        .iter()
        .filter(|theme| matches!(theme.identity(), ThemeIdentity::UserDefined { .. }))
    {
        known.insert(theme_hash(theme), theme.id());
    }
    let mut id_map = HashMap::new();
    for theme in imported
        .themes
        .iter()
        .filter(|theme| matches!(theme.identity(), ThemeIdentity::UserDefined { .. }))
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
