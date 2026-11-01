use crate::shared::application::PortResult;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{shared::application::ApplicationPortError, shared::domain::GroupId};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "application_visibility_groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub application_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::ApplicationId",
        to = "super::application::Column::Id",
        on_delete = "Cascade"
    )]
    Application,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn group_id(self) -> PortResult<GroupId> {
        GroupId::try_from(self.group_id).map_err(ApplicationPortError::from)
    }
}

impl ActiveModel {
    pub fn for_application(application_id: Uuid, group_id: &GroupId) -> Self {
        Self {
            application_id: Set(application_id),
            group_id: Set(group_id.as_str().to_owned()),
        }
    }
}
