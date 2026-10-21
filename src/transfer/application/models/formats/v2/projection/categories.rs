use crate::{
    dashboard::domain::{Bookmark, DashboardCategory},
    transfer::application::models::formats::{
        bookmark_hash, category_hash,
        v2::{BookmarkExport, CategoryExport},
    },
};

pub(super) fn export(categories: &[DashboardCategory]) -> Vec<CategoryExport> {
    categories.iter().map(export_category).collect()
}

fn export_category(category: &DashboardCategory) -> CategoryExport {
    let bookmarks = category.bookmarks().iter().map(export_bookmark).collect();
    let name = category.name().to_owned();
    let shelved = matches!(category, DashboardCategory::Shelved(_));
    let hash = category_hash(category);
    CategoryExport {
        hash,
        name,
        shelved,
        bookmarks,
    }
}

fn export_bookmark(bookmark: &Bookmark) -> BookmarkExport {
    let name = bookmark.name().to_owned();
    let icon = bookmark.icon().as_str().to_owned();
    let address = bookmark.address().as_str().to_owned();
    let hash = bookmark_hash(bookmark);
    BookmarkExport {
        hash,
        name,
        icon,
        address,
    }
}
