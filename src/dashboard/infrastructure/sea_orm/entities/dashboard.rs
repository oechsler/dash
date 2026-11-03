use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{
    dashboard::domain::{DashboardCategory, UserDashboard},
    shared::application::{ApplicationPortError, PortResult},
    shared::domain::{CollectionSortOrder, UserId},
    shared::infrastructure::{DatabaseAdapterError as InfrastructureError, DatabaseError},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dashboards")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_id: String,
    pub categories_sort_order: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::user::infrastructure::sea_orm::entities::user::Entity",
        from = "Column::OwnerId",
        to = "crate::user::infrastructure::sea_orm::entities::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_domain(self, categories: Vec<DashboardCategory>) -> PortResult<UserDashboard> {
        let sort_order = CollectionSortOrder::try_from(self.categories_sort_order.as_str())
            .map_err(|_| {
                InfrastructureError::persistence(
                    "decode dashboard sort order",
                    DatabaseError::invalid_value(
                        "dashboard categories_sort_order",
                        self.categories_sort_order.clone(),
                    ),
                )
            })?;
        Ok(UserDashboard::restore(
            UserId::try_from(self.owner_id).map_err(ApplicationPortError::from)?,
            categories,
            sort_order,
        ))
    }
}

impl ActiveModel {
    pub fn from_domain(dashboard: &UserDashboard) -> Self {
        Self {
            owner_id: Set(dashboard.owner().as_str().to_owned()),
            categories_sort_order: Set(dashboard.categories_sort_order().as_str().to_owned()),
        }
    }
}
