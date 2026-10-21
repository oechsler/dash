use super::{Category, CategoryError, DashboardCategory, DashboardError, ShelvedCategory};

#[test]
fn category_rejects_blank_names_without_mutating() {
    let mut category = Category::new("Work").unwrap();
    assert_eq!(
        category.rename("  "),
        Err(DashboardError::Category(CategoryError::EmptyName {
            value: "  ".to_owned()
        }))
    );
    assert_eq!(category.name(), "Work");
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
