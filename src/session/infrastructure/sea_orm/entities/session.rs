use chrono::{DateTime, Utc};
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use std::{net::IpAddr, str::FromStr};

use crate::{
    session::domain::{LongLived, Session, SessionId, SessionMetadata, ShortLived, UserSession},
    shared::domain::UserId,
    shared::{
        application::{ApplicationPortError, PortResult},
        infrastructure::DatabaseAdapterError as InfrastructureError,
    },
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

impl Model {
    pub fn into_domain(self) -> PortResult<UserSession> {
        let owner = UserId::try_from(self.owner_id).map_err(ApplicationPortError::from)?;
        let ip = self
            .ip_address
            .map(|value| {
                IpAddr::from_str(&value)
                    .map_err(|error| InfrastructureError::database("decode session IP", error))
            })
            .transpose()?;
        let metadata = SessionMetadata::new(self.browser, self.operating_system, ip);
        if self.long_lived {
            Ok(UserSession::LongLived(
                Session::<LongLived>::restore(
                    SessionId::from(self.id),
                    owner,
                    metadata,
                    self.issued_at,
                    self.last_accessed_at,
                    self.expires_at,
                    self.terminated_at,
                )
                .map_err(ApplicationPortError::from)?,
            ))
        } else {
            Ok(UserSession::ShortLived(
                Session::<ShortLived>::restore(
                    SessionId::from(self.id),
                    owner,
                    metadata,
                    self.issued_at,
                    self.last_accessed_at,
                    self.expires_at,
                    self.terminated_at,
                )
                .map_err(ApplicationPortError::from)?,
            ))
        }
    }
}

impl ActiveModel {
    pub fn from_domain(value: &UserSession) -> Self {
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
