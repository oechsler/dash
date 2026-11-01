use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::shared::domain::UserId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn for_external_identity(user_id: &UserId, username: &str) -> Self {
        Self {
            id: Set(user_id.as_str().to_owned()),
            username: Set(username.to_owned()),
            first_name: Set(None),
            last_name: Set(None),
            profile_picture: Set(None),
        }
    }
}
