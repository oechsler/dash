use crate::icon::domain::IconReference;

use super::{Bookmark, BookmarkError, DashboardError, WebAddress};

#[test]
fn bookmark_can_be_renamed_and_change_icon_and_address() {
    let mut bookmark = Bookmark::new(
        "Docs",
        IconReference::try_from("pack:book").unwrap(),
        WebAddress::try_from("https://docs.example").unwrap(),
    )
    .unwrap();

    bookmark.rename("  Team docs  ").unwrap();
    bookmark.change_icon(IconReference::try_from("https://assets.example/docs.svg").unwrap());
    bookmark.change_address(WebAddress::try_from("https://team.example/docs").unwrap());

    assert_eq!(bookmark.name(), "Team docs");
    assert_eq!(bookmark.icon().as_str(), "https://assets.example/docs.svg");
    assert_eq!(bookmark.address().as_str(), "https://team.example/docs");
}

#[test]
fn bookmark_rejects_an_empty_name_without_mutating() {
    let mut bookmark = Bookmark::new(
        "Docs",
        IconReference::try_from("pack:book").unwrap(),
        WebAddress::try_from("https://docs.example").unwrap(),
    )
    .unwrap();

    assert!(matches!(
        bookmark.rename("  "),
        Err(DashboardError::Bookmark(BookmarkError::EmptyName { .. }))
    ));
    assert_eq!(bookmark.name(), "Docs");
}
