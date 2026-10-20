use crate::{
    dashboard::domain::{Bookmark, Category, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationError,
    transfer::application::models::formats::v2::CategoryExport,
    user::domain::UserId,
};

pub(super) fn build(
    owner: &UserId,
    records: Vec<CategoryExport>,
) -> Result<UserDashboard, ApplicationError> {
    let mut dashboard = UserDashboard::new(owner.clone());
    for record in records {
        let mut category = Category::new(record.name)?;
        if record.shelved {
            category.shelve();
        }
        for bookmark in record.bookmarks {
            category.add_bookmark(Bookmark::new(
                bookmark.name,
                IconReference::parse(bookmark.icon)?,
                WebAddress::parse(bookmark.address)?,
            )?);
        }
        dashboard.add_category(category);
    }
    Ok(dashboard)
}
