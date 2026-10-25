use std::collections::{HashMap, HashSet};

use crate::{
    dashboard::domain::UserDashboard,
    preferences::domain::UserPreferences,
    shared::application::ApplicationError,
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
        let mut known: Vec<_> = target
            .applications
            .iter()
            .map(UserDataSnapshot::application_hash)
            .collect();
        for application in &imported.applications {
            let hash = UserDataSnapshot::application_hash(application);
            if !known.contains(&hash) {
                target.applications.push(application.clone());
                known.push(hash);
            }
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
            theme_ids,
            &target.themes,
        )?;
        self.remap_theme(
            &mut preferences,
            ThemeMode::Dark,
            imported_preferences,
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
        theme_ids: &ThemeIdMap,
        themes: &[Theme],
    ) -> Result<(), ApplicationError> {
        let Some(imported_id) = imported_preferences.theme_id(mode) else {
            return Ok(());
        };
        let Some(target_id) = theme_ids.get(&imported_id) else {
            return Ok(());
        };
        let theme = themes
            .iter()
            .find(|theme| theme.id() == *target_id)
            .ok_or(ApplicationError::NotFound("theme"))?;
        preferences.select_theme(theme)?;
        Ok(())
    }
}
