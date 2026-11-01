use sea_orm::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::preferences::domain::UserPreferences;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_recent_applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub application_id: Uuid,
    pub position: i32,
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
    #[sea_orm(
        belongs_to = "crate::catalog::infrastructure::sea_orm::entities::application::Entity",
        from = "Column::ApplicationId",
        to = "crate::catalog::infrastructure::sea_orm::entities::application::Column::Id",
        on_delete = "Cascade"
    )]
    Application,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn from_preferences(
        preferences: &UserPreferences,
        application_id: Uuid,
        position: i32,
    ) -> Self {
        Self {
            owner_id: Set(preferences.user_id().as_str().to_owned()),
            application_id: Set(application_id),
            position: Set(position),
        }
    }
}
