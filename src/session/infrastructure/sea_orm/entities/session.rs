use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use std::{net::IpAddr, str::FromStr};

use crate::{
    session::domain::{LongLived, Session, SessionId, SessionMetadata, ShortLived, UserSession},
    shared::domain::UserId,
    shared::{application::ApplicationPortError, infrastructure::DatabaseError},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: String,
    pub long_lived: bool,
    pub browser: Option<String>,
    pub operating_system: Option<String>,
    pub ip_address: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub terminated_at: Option<DateTime<Utc>>,
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

impl TryFrom<Model> for UserSession {
    type Error = ApplicationPortError;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        let owner = UserId::try_from(model.owner_id).map_err(ApplicationPortError::from)?;
        let ip = model
            .ip_address
            .map(|value| {
                IpAddr::from_str(&value).map_err(|error| {
                    ApplicationPortError::adapter(
                        "decode session IP",
                        super::super::super::error::InfrastructureError::from(
                            DatabaseError::source(error),
                        ),
                    )
                })
            })
            .transpose()?;
        let metadata = SessionMetadata::new(model.browser, model.operating_system, ip);
        if model.long_lived {
            Ok(UserSession::LongLived(
                Session::<LongLived>::restore(
                    SessionId::from(model.id),
                    owner,
                    metadata,
                    model.issued_at,
                    model.last_accessed_at,
                    model.expires_at,
                    model.terminated_at,
                )
                .map_err(ApplicationPortError::from)?,
            ))
        } else {
            Ok(UserSession::ShortLived(
                Session::<ShortLived>::restore(
                    SessionId::from(model.id),
                    owner,
                    metadata,
                    model.issued_at,
                    model.last_accessed_at,
                    model.expires_at,
                    model.terminated_at,
                )
                .map_err(ApplicationPortError::from)?,
            ))
        }
    }
}

impl From<&UserSession> for ActiveModel {
    fn from(value: &UserSession) -> Self {
        let metadata = value.metadata();
        Self {
            id: Set(value.id().as_uuid()),
            owner_id: Set(value.owner().as_str().to_owned()),
            long_lived: Set(value.is_long_lived()),
            browser: Set(metadata.browser().map(str::to_owned)),
            operating_system: Set(metadata.operating_system().map(str::to_owned)),
            ip_address: Set(metadata.ip_address().map(|ip| ip.to_string())),
            issued_at: Set(value.issued_at()),
            last_accessed_at: Set(value.last_accessed_at()),
            expires_at: Set(value.expires_at()),
            terminated_at: Set(value.terminated_at()),
        }
    }
}
