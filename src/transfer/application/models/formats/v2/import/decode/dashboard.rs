use crate::{
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
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
        for bookmark in record.bookmarks {
            category.add_bookmark(Bookmark::new(
                bookmark.name,
                IconReference::parse(bookmark.icon)?,
                WebAddress::parse(bookmark.address)?,
            )?);
        }
        if record.shelved {
            dashboard.add_category(ShelvedCategory::from(category));
        } else {
            dashboard.add_category(category);
        }
    }
    Ok(dashboard)
}
