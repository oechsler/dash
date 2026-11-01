use crate::shared::application::PortResult;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::{
    shared::application::ApplicationPortError,
    shared::domain::UserId,
    theme::domain::{Color, Theme, ThemeId, ThemeMode, ThemePalette},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "themes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub owner_id: Option<String>,
    pub name: String,
    pub mode: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub tertiary_color: String,
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
    pub fn into_domain(self, owner: &UserId) -> PortResult<Theme> {
        let id = Uuid::parse_str(&self.id).map_err(|error| {
            ApplicationPortError::adapter(
                "decode theme id",
                super::super::super::error::InfrastructureError::from(
                    crate::shared::infrastructure::DatabaseError::source(error),
                ),
            )
        })?;
        let mode = match self.mode.as_str() {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => {
                return Err(ApplicationPortError::adapter(
                    "decode theme mode",
                    super::super::super::error::InfrastructureError::invalid_value(
                        "mode", self.mode,
                    ),
                ));
            }
        };
        let palette = ThemePalette::new(
            Color::try_from(self.primary_color).map_err(ApplicationPortError::from)?,
            Color::try_from(self.secondary_color).map_err(ApplicationPortError::from)?,
            Color::try_from(self.tertiary_color).map_err(ApplicationPortError::from)?,
        );
        Theme::restore_user_defined(id, owner.clone(), self.name, mode, palette)
            .map_err(ApplicationPortError::from)
    }
}

impl TryFrom<&Theme> for ActiveModel {
    type Error = ApplicationPortError;

    fn try_from(theme: &Theme) -> PortResult<Self> {
        let ThemeId::UserDefined(id) = theme.id() else {
            return Err(ApplicationPortError::NotFound("user-defined theme"));
        };
        let owner = match theme.identity() {
            crate::theme::domain::ThemeIdentity::UserDefined { owner, .. } => owner.as_str(),
            crate::theme::domain::ThemeIdentity::Global(_) => unreachable!(),
        };
        Ok(Self {
            id: Set(id.to_string()),
            owner_id: Set(Some(owner.to_owned())),
            name: Set(theme.name().to_owned()),
            mode: Set(match theme.mode() {
                ThemeMode::Light => "light".to_owned(),
                ThemeMode::Dark => "dark".to_owned(),
            }),
            primary_color: Set(theme.palette().primary().as_str().to_owned()),
            secondary_color: Set(theme.palette().secondary().as_str().to_owned()),
            tertiary_color: Set(theme.palette().tertiary().as_str().to_owned()),
        })
    }
}
