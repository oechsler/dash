use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{
    preferences::domain::{Language, TimeZone, UserPreferences},
    shared::domain::UserId,
    shared::{application::ApplicationPortError, infrastructure::DatabaseError},
    theme::domain::{ThemeId, ThemeMode},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_preferences")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub owner_id: String,
    pub light_theme_id: Option<String>,
    pub dark_theme_id: Option<String>,
    pub language: String,
    pub time_zone: String,
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
    pub fn into_domain(
        self,
        owner: &UserId,
        recent_application_ids: Vec<crate::catalog::domain::ApplicationId>,
    ) -> Result<UserPreferences, ApplicationPortError> {
        let parse_theme = |value: Option<String>| -> Result<Option<ThemeId>, ApplicationPortError> {
            value
                .map(|value| ThemeId::try_from(value.as_str()).map_err(ApplicationPortError::from))
                .transpose()
        };
        let light = parse_theme(self.light_theme_id)?;
        let dark = parse_theme(self.dark_theme_id)?;
        let language = Language::try_from(self.language.as_str()).map_err(|_| {
            ApplicationPortError::adapter(
                "decode language",
                super::super::super::error::InfrastructureError::from(
                    DatabaseError::invalid_value("language", self.language.clone()),
                ),
            )
        })?;
        let timezone = TimeZone::try_from(self.time_zone).map_err(ApplicationPortError::from)?;
        Ok(UserPreferences::restore(
            owner.clone(),
            light,
            dark,
            language,
            recent_application_ids,
            timezone,
        ))
    }
}

impl From<&UserPreferences> for ActiveModel {
    fn from(value: &UserPreferences) -> Self {
        Self {
            owner_id: Set(value.user_id().as_str().to_owned()),
            light_theme_id: Set(value.theme_id(ThemeMode::Light).map(String::from)),
            dark_theme_id: Set(value.theme_id(ThemeMode::Dark).map(String::from)),
            language: Set(value.language().as_str().to_owned()),
            time_zone: Set(value.time_zone().as_str().to_owned()),
        }
    }
}
