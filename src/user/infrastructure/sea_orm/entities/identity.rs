use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::shared::domain::UserId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "external_identities")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub issuer: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub subject: String,
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn for_external_identity(issuer: &str, subject: &str, user_id: &UserId) -> Self {
        Self {
            issuer: Set(issuer.to_owned()),
            subject: Set(subject.to_owned()),
            user_id: Set(user_id.as_str().to_owned()),
        }
    }
}
