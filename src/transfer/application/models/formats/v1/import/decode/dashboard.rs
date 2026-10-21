use crate::{
    dashboard::domain::{Bookmark, Category, ShelvedCategory, UserDashboard, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::UserId,
};

use super::super::model::GoCategoryV1;

pub(super) fn build(
    owner: &UserId,
    records: Vec<GoCategoryV1>,
) -> Result<UserDashboard, ApplicationError> {
    let mut dashboard = UserDashboard::new(owner.clone());
    for record in records {
        let mut category = Category::new(record.display_name)?;
        for bookmark in record.bookmarks {
            category.add_bookmark(Bookmark::new(
                bookmark.display_name,
                IconReference::parse(bookmark.icon)?,
                WebAddress::parse(bookmark.url)?,
            )?);
        }
        if record.is_shelved {
            dashboard.add_category(ShelvedCategory::from(category));
        } else {
            dashboard.add_category(category);
        }
    }
    Ok(dashboard)
}
