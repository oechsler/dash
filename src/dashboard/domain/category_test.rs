use super::{BookmarkId, Category, CategoryError, DashboardCategory, ShelvedCategory};

#[test]
fn category_rejects_blank_names_without_mutating() {
    let mut category = Category::new("Work").unwrap();
    assert_eq!(
        category.rename("  "),
        Err(CategoryError::EmptyName {
            value: "  ".to_owned()
        })
    );
    assert_eq!(category.name(), "Work");
}

#[test]
fn category_reports_its_own_invalid_bookmark_order_error() {
    let mut category = Category::new("Work").unwrap();

    assert_eq!(
        category.reorder_bookmarks(&[BookmarkId::new()]),
        Err(CategoryError::InvalidBookmarkOrder)
    );
}

#[test]
fn category_changes_between_distinct_active_and_shelved_types() {
    let category = Category::new("Work").unwrap();
    let shelved: ShelvedCategory = category.into();
    assert_eq!(shelved.name(), "Work");
    let category = shelved.unshelve();
    assert_eq!(category.name(), "Work");
    assert!(matches!(
        DashboardCategory::from(category),
        DashboardCategory::Active(_)
    ));
}
