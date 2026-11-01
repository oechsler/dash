use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{
    dashboard::domain::{Bookmark, BookmarkId, WebAddress},
    icon::domain::IconReference,
    shared::application::ApplicationPortError,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dashboard_bookmarks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub position: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_delete = "Cascade"
    )]
    Category,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for Bookmark {
    type Error = ApplicationPortError;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        Bookmark::restore(
            BookmarkId::from(model.id),
            model.name,
            IconReference::try_from(model.icon).map_err(ApplicationPortError::from)?,
            WebAddress::try_from(model.address).map_err(ApplicationPortError::from)?,
            model.last_accessed_at,
        )
        .map_err(ApplicationPortError::from)
    }
}

impl ActiveModel {
    pub fn from_domain(bookmark: &Bookmark, category_id: Uuid, position: i32) -> Self {
        Self {
            id: Set(bookmark.id().as_uuid()),
            category_id: Set(category_id),
            name: Set(bookmark.name().to_owned()),
            icon: Set(bookmark.icon().as_str().to_owned()),
            address: Set(bookmark.address().as_str().to_owned()),
            last_accessed_at: Set(bookmark.last_accessed_at()),
            position: Set(position),
        }
    }
}
