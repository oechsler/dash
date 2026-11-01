use crate::shared::application::PortResult;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{
    dashboard::domain::{Category, CategoryId, DashboardCategory, ShelvedCategory},
    shared::domain::UserId,
    shared::{application::ApplicationPortError, domain::CollectionSortOrder},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dashboard_categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: String,
    pub name: String,
    pub is_shelved: bool,
    pub sort_order: String,
    pub position: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dashboard::Entity",
        from = "Column::OwnerId",
        to = "super::dashboard::Column::OwnerId",
        on_delete = "Cascade"
    )]
    Dashboard,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_domain(
        self,
        bookmarks: Vec<crate::dashboard::domain::Bookmark>,
    ) -> PortResult<DashboardCategory> {
        let sort_order = CollectionSortOrder::try_from(self.sort_order.as_str()).map_err(|_| {
            ApplicationPortError::adapter(
                "decode category sort order",
                super::super::super::error::InfrastructureError::invalid_value(
                    "category sort_order",
                    self.sort_order.clone(),
                ),
            )
        })?;
        let category =
            Category::restore(CategoryId::from(self.id), self.name, bookmarks, sort_order)
                .map_err(ApplicationPortError::from)?;
        Ok(if self.is_shelved {
            DashboardCategory::Shelved(ShelvedCategory::from(category))
        } else {
            DashboardCategory::Active(category)
        })
    }
}

impl ActiveModel {
    pub fn from_domain(category: &DashboardCategory, owner: &UserId, position: i32) -> Self {
        let (name, is_shelved) = match category {
            DashboardCategory::Active(category) => (category.name(), false),
            DashboardCategory::Shelved(category) => (category.name(), true),
        };
        Self {
            id: Set(category.id().as_uuid()),
            owner_id: Set(owner.as_str().to_owned()),
            name: Set(name.to_owned()),
            is_shelved: Set(is_shelved),
            sort_order: Set(category.bookmarks_sort_order().as_str().to_owned()),
            position: Set(position),
        }
    }
}
