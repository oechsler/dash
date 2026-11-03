use super::super::error::InfrastructureError;
use super::entities::session::{self, Entity};
use crate::shared::application::PortResult;
use crate::{
    session::{
        application::ports::{SessionReader, SessionWriter},
        domain::{SessionId, UserSession},
    },
    shared::domain::UserId,
};
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SessionRepository {
    connection: Arc<DatabaseConnection>,
}

impl SessionRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }
}

#[async_trait]
impl SessionReader for SessionRepository {
    async fn load(&self, owner: &UserId, id: SessionId) -> PortResult<Option<UserSession>> {
        let row = Entity::find_by_id(id.as_uuid())
            .filter(session::Column::OwnerId.eq(owner.as_str()))
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load session", e))?;
        row.map(|model| model.into_domain()).transpose()
    }

    async fn load_by_id(&self, id: SessionId) -> PortResult<Option<UserSession>> {
        Entity::find_by_id(id.as_uuid())
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load session by id", e))?
            .map(|model| model.into_domain())
            .transpose()
    }

    async fn list_for(&self, owner: &UserId) -> PortResult<Vec<UserSession>> {
        Entity::find()
            .filter(session::Column::OwnerId.eq(owner.as_str()))
            .order_by_desc(session::Column::LastAccessedAt)
            .all(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("list sessions", e))?
            .into_iter()
            .map(|model| model.into_domain())
            .collect()
    }
}

#[async_trait]
impl SessionWriter for SessionRepository {
    async fn save(&self, value: &UserSession) -> PortResult<()> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin session save", e))?;

        Entity::insert(session::ActiveModel::from_domain(value))
            .on_conflict(
                OnConflict::column(session::Column::Id)
                    .update_columns([
                        session::Column::OwnerId,
                        session::Column::LongLived,
                        session::Column::Browser,
                        session::Column::OperatingSystem,
                        session::Column::IpAddress,
                        session::Column::IssuedAt,
                        session::Column::LastAccessedAt,
                        session::Column::ExpiresAt,
                        session::Column::TerminatedAt,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(&tx)
            .await
            .map_err(|e| InfrastructureError::database("save session", e))?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit session save", e))
    }

    async fn delete(&self, owner: &UserId, id: SessionId) -> PortResult<()> {
        Entity::delete_many()
            .filter(session::Column::Id.eq(id.as_uuid()))
            .filter(session::Column::OwnerId.eq(owner.as_str()))
            .exec(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("delete session", e))?;
        Ok(())
    }

    async fn delete_expired(&self, now: chrono::DateTime<chrono::Utc>) -> PortResult<()> {
        Entity::delete_many()
            .filter(session::Column::ExpiresAt.lte(now))
            .exec(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("delete expired sessions", e))?;
        Ok(())
    }
}
