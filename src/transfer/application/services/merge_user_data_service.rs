use std::collections::{HashMap, HashSet};

use crate::{
    dashboard::domain::UserDashboard,
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
    shared::domain::CollectionSortOrder,
    theme::domain::{Theme, ThemeId, ThemeIdentity, ThemeMode},
    transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub struct MergeUserDataService;

type ThemeIdMap = HashMap<ThemeId, ThemeId>;

impl MergeUserDataService {
    pub fn merge(
        &self,
        target: &mut UserDataSnapshot,
        imported: &UserDataSnapshot,
        owner: &UserId,
        is_admin: bool,
    ) -> Result<(), ApplicationError> {
        let theme_ids = self.merge_themes(target, imported)?;
        self.merge_dashboard(target, imported, owner)?;
        if is_admin {
            self.merge_applications(target, imported)?;
        }
        self.merge_preferences(target, imported, owner, &theme_ids)
    }

    fn merge_themes(
        &self,
        target: &mut UserDataSnapshot,
        imported: &UserDataSnapshot,
    ) -> Result<ThemeIdMap, ApplicationError> {
        let mut known = HashMap::new();
        for theme in target
            .themes
            .iter()
            .filter(|theme| matches!(theme.identity(), ThemeIdentity::UserDefined { .. }))
        {
            known.insert(UserDataSnapshot::theme_hash(theme), theme.id());
        }

        let mut id_map = HashMap::new();
        for theme in imported
            .themes
            .iter()
            .filter(|theme| matches!(theme.identity(), ThemeIdentity::UserDefined { .. }))
        {
            let key = UserDataSnapshot::theme_hash(theme);
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

    fn merge_dashboard(
        &self,
        target: &mut UserDataSnapshot,
        imported: &UserDataSnapshot,
        owner: &UserId,
    ) -> Result<(), ApplicationError> {
        let target_dashboard = target
            .dashboard
            .get_or_insert_with(|| UserDashboard::new(owner.clone()));
        let Some(imported_dashboard) = &imported.dashboard else {
            return Ok(());
        };

        for imported_category in imported_dashboard.categories() {
            let imported_hash = UserDataSnapshot::category_hash(imported_category);
            let matching_id = target_dashboard
                .categories()
                .iter()
                .find(|category| UserDataSnapshot::category_hash(category) == imported_hash)
                .map(|category| category.id());
            let Some(category_id) = matching_id else {
                target_dashboard.add_category(imported_category.clone());
                continue;
            };

            let mut known_bookmarks: HashSet<String> = target_dashboard
                .categories()
                .iter()
                .find(|category| category.id() == category_id)
                .into_iter()
                .flat_map(|category| category.bookmarks())
                .map(UserDataSnapshot::bookmark_hash)
                .collect();
            for bookmark in imported_category.bookmarks() {
                if known_bookmarks.insert(UserDataSnapshot::bookmark_hash(bookmark)) {
                    target_dashboard.add_bookmark(category_id, bookmark.clone())?;
                }
            }
        }
        Ok(())
    }

    fn merge_applications(
        &self,
        target: &mut UserDataSnapshot,
        imported: &UserDataSnapshot,
    ) -> Result<(), ApplicationError> {
        let mut known: HashMap<_, _> = target
            .applications
            .iter()
            .map(|application| {
                (
                    UserDataSnapshot::application_hash(application),
                    application.id(),
                )
            })
            .collect();
        let mut imported_id_map = HashMap::new();
        for application in &imported.applications {
            let hash = UserDataSnapshot::application_hash(application);
            let target_id = if let Some(id) = known.get(&hash) {
                *id
            } else {
                target.applications.push(application.clone());
                known.insert(hash, application.id());
                application.id()
            };
            imported_id_map.insert(application.id(), target_id);
        }

        if let Some(imported_settings) = &imported.application_catalog_settings {
            let mut settings = target
                .application_catalog_settings
                .clone()
                .unwrap_or_default();
            settings.set_sort_order(imported_settings.sort_order());
            if imported_settings.sort_order() == CollectionSortOrder::UserDefined {
                let available_ids = target
                    .applications
                    .iter()
                    .map(|application| application.id())
                    .collect::<Vec<_>>();
                let available: HashSet<_> = available_ids.iter().copied().collect();
                let mut ordered_ids = Vec::new();
                for id in imported_settings.manual_order() {
                    if let Some(target_id) = imported_id_map.get(id).copied()
                        && available.contains(&target_id)
                        && !ordered_ids.contains(&target_id)
                    {
                        ordered_ids.push(target_id);
                    }
                }
                for id in settings.manual_order() {
                    if available.contains(id) && !ordered_ids.contains(id) {
                        ordered_ids.push(*id);
                    }
                }
                for id in &available_ids {
                    if !ordered_ids.contains(id) {
                        ordered_ids.push(*id);
                    }
                }
                settings.set_manual_order(ordered_ids, &available_ids)?;
            }
            target.application_catalog_settings = Some(settings);
        }
        Ok(())
    }

    fn merge_preferences(
        &self,
        target: &mut UserDataSnapshot,
        imported: &UserDataSnapshot,
        owner: &UserId,
        theme_ids: &ThemeIdMap,
    ) -> Result<(), ApplicationError> {
        let Some(imported_preferences) = imported.preferences.as_ref() else {
            return Ok(());
        };
        let mut preferences = UserPreferences::for_user(owner.clone());
        preferences.set_language(imported_preferences.language());
        preferences.set_time_zone(imported_preferences.time_zone().clone());
        self.remap_theme(
            &mut preferences,
            ThemeMode::Light,
            imported_preferences,
            imported.imported_theme_names.as_ref(),
            theme_ids,
            &target.themes,
        )?;
        self.remap_theme(
            &mut preferences,
            ThemeMode::Dark,
            imported_preferences,
            imported.imported_theme_names.as_ref(),
            theme_ids,
            &target.themes,
        )?;
        target.preferences = Some(preferences);
        Ok(())
    }

    fn remap_theme(
        &self,
        preferences: &mut UserPreferences,
        mode: ThemeMode,
        imported_preferences: &UserPreferences,
        imported_theme_names: Option<&crate::transfer::application::models::ImportedThemeNames>,
        theme_ids: &ThemeIdMap,
        themes: &[Theme],
    ) -> Result<(), ApplicationError> {
        if let Some(imported_id) = imported_preferences.theme_id(mode) {
            let Some(target_id) = theme_ids.get(&imported_id) else {
                return Ok(());
            };
            let theme = themes
                .iter()
                .find(|theme| theme.id() == *target_id)
                .ok_or(ApplicationError::NotFound("theme"))?;
            preferences.select_theme(theme)?;
            return Ok(());
        }

        let name = imported_theme_names.and_then(|names| match mode {
            ThemeMode::Light => names.light.as_deref(),
            ThemeMode::Dark => names.dark.as_deref(),
        });
        let Some(name) = name else { return Ok(()) };
        if let Some(theme) = themes.iter().find(|theme| {
            theme.name() == name
                && theme.mode() == mode
                && matches!(theme.identity(), ThemeIdentity::Global(_))
        }) {
            preferences.select_theme(theme)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        theme::domain::{Color, GlobalThemeId, ThemePalette},
        transfer::application::models::ImportedThemeNames,
    };

    fn global_dark_theme() -> Theme {
        Theme::global()
            .id(GlobalThemeId::new("destination-ocean").unwrap())
            .name("Ocean")
            .mode(ThemeMode::Dark)
            .palette(ThemePalette::new(
                Color::parse("#111111").unwrap(),
                Color::parse("#222222").unwrap(),
                Color::parse("#333333").unwrap(),
            ))
            .call()
            .unwrap()
    }

    #[test]
    fn import_reselects_a_matching_global_theme_by_name_and_mode() {
        let owner = UserId::new("user-1").unwrap();
        let theme = global_dark_theme();
        let mut target = UserDataSnapshot {
            themes: vec![theme.clone()],
            ..UserDataSnapshot::default()
        };
        let imported = UserDataSnapshot {
            preferences: Some(UserPreferences::for_user(owner.clone())),
            imported_theme_names: Some(ImportedThemeNames {
                light: None,
                dark: Some("Ocean".to_owned()),
            }),
            ..UserDataSnapshot::default()
        };

        MergeUserDataService
            .merge(&mut target, &imported, &owner, false)
            .unwrap();

        assert_eq!(
            target.preferences.unwrap().theme_id(ThemeMode::Dark),
            Some(theme.id())
        );
    }

    #[test]
    fn import_uses_the_default_when_a_global_theme_is_unavailable() {
        let owner = UserId::new("user-1").unwrap();
        let mut target = UserDataSnapshot::default();
        let imported = UserDataSnapshot {
            preferences: Some(UserPreferences::for_user(owner.clone())),
            imported_theme_names: Some(ImportedThemeNames {
                light: None,
                dark: Some("Ocean".to_owned()),
            }),
            ..UserDataSnapshot::default()
        };

        MergeUserDataService
            .merge(&mut target, &imported, &owner, false)
            .unwrap();

        assert_eq!(target.preferences.unwrap().theme_id(ThemeMode::Dark), None);
    }
}
