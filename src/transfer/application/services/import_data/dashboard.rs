use std::collections::HashSet;

use crate::{
    dashboard::domain::{Category, UserDashboard},
    shared::application::ApplicationError,
    transfer::application::models::{
        UserDataSnapshot,
        formats::{bookmark_hash, category_hash},
    },
    user::domain::UserId,
};

pub(super) fn merge(
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
        let imported_hash = category_hash(imported_category);
        let matching_id = target_dashboard
            .categories()
            .iter()
            .find(|category| category_hash(category) == imported_hash)
            .map(Category::id);
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
            .map(bookmark_hash)
            .collect();
        for bookmark in imported_category.bookmarks() {
            if known_bookmarks.insert(bookmark_hash(bookmark)) {
                target_dashboard.add_bookmark(category_id, bookmark.clone())?;
            }
        }
    }
    Ok(())
}
