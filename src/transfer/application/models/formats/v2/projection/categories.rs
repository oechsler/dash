use crate::{
    dashboard::domain::{Bookmark, Category},
    transfer::application::models::formats::{
        content_hash,
        v2::{BookmarkExport, CategoryExport},
    },
};

pub(super) fn export(categories: &[Category]) -> Vec<CategoryExport> {
    categories.iter().map(export_category).collect()
}

fn export_category(category: &Category) -> CategoryExport {
    let bookmarks = category.bookmarks().iter().map(export_bookmark).collect();
    let name = category.name().to_owned();
    let shelved = category.is_shelved();
    let hash = content_hash(&[&name, if shelved { "true" } else { "false" }]);
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
    let hash = content_hash(&[&icon, &name, &address]);
    BookmarkExport {
        hash,
        name,
        icon,
        address,
    }
}
