use super::{Category, CategoryError};

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
fn category_can_be_shelved_and_unshelved() {
    let mut category = Category::new("Work").unwrap();
    category.shelve();
    assert!(category.is_shelved());
    category.unshelve();
    assert!(!category.is_shelved());
}
