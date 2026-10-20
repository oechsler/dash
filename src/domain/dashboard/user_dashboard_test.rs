use crate::domain::dashboard::{
    Bookmark, Category, CategoryId, DashboardError, UserDashboard, WebAddress,
};
use crate::domain::icon::IconReference;
use crate::domain::user::UserId;

#[test]
fn dashboard_owns_categories_and_bookmarks() {
    let owner = UserId::new("user-1").unwrap();
    let mut dashboard = UserDashboard::new(owner.clone());
    let mut category = Category::new("  Work  ").unwrap();
    let category_id = category.id();
    category.add_bookmark(
        Bookmark::new(
            "Docs",
            IconReference::parse("mdi:book").unwrap(),
            WebAddress::parse("https://docs.example").unwrap(),
        )
        .unwrap(),
    );
    dashboard.add_category(category);

    assert_eq!(dashboard.owner(), &owner);
    assert_eq!(dashboard.categories()[0].name(), "Work");
    assert_eq!(dashboard.categories()[0].bookmarks().len(), 1);
    assert!(dashboard.category_mut(category_id).is_some());
}

#[test]
fn dashboard_can_move_and_remove_bookmarks_across_its_categories() {
    let mut dashboard = UserDashboard::new(UserId::new("user-1").unwrap());
    let mut work = Category::new("Work").unwrap();
    let archive = Category::new("Archive").unwrap();
    let archive_id = archive.id();
    let bookmark = Bookmark::new(
        "Docs",
        IconReference::parse("pack:book").unwrap(),
        WebAddress::parse("https://docs.example").unwrap(),
    )
    .unwrap();
    let bookmark_id = bookmark.id();
    work.add_bookmark(bookmark);
    dashboard.add_category(work);
    dashboard.add_category(archive);

    dashboard.move_bookmark(bookmark_id, archive_id).unwrap();
    assert!(dashboard.categories()[0].bookmarks().is_empty());
    assert_eq!(dashboard.categories()[1].bookmarks()[0].id(), bookmark_id);

    let removed = dashboard.remove_bookmark(bookmark_id).unwrap();
    assert_eq!(removed.id(), bookmark_id);
    assert!(dashboard.categories()[1].bookmarks().is_empty());
    assert_eq!(
        dashboard.move_bookmark(bookmark_id, archive_id),
        Err(DashboardError::BookmarkNotFound { bookmark_id })
    );
    let missing_category_id = CategoryId::new();
    assert_eq!(
        dashboard.move_bookmark(bookmark_id, missing_category_id),
        Err(DashboardError::CategoryNotFound {
            category_id: missing_category_id
        })
    );
}

#[test]
fn removing_category_returns_it_with_bookmarks_for_cascade_removal() {
    let mut dashboard = UserDashboard::new(UserId::new("user-1").unwrap());
    let mut category = Category::new("Work").unwrap();
    let category_id = category.id();
    category.add_bookmark(
        Bookmark::new(
            "Docs",
            IconReference::parse("pack:book").unwrap(),
            WebAddress::parse("https://docs.example").unwrap(),
        )
        .unwrap(),
    );
    dashboard.add_category(category);

    let removed = dashboard.remove_category(category_id).unwrap();
    assert_eq!(removed.bookmarks().len(), 1);
    assert!(dashboard.categories().is_empty());
}
