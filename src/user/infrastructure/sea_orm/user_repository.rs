use crate::shared::application::PortResult;
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use std::sync::Arc;
use uuid::Uuid;

use super::super::error::InfrastructureError;
use super::entities::{identity, user};
use crate::{
    shared::{application::ApplicationPortError, domain::UserId, infrastructure::DatabaseError},
    user::application::ports::{IdentityWriter, ResolvedUser, UserWriter},
};

#[derive(Clone, Debug)]
pub struct UserRepository {
    connection: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }
}

#[async_trait]
impl IdentityWriter for UserRepository {
    async fn resolve_or_create(&self, issuer: &str, subject: &str) -> PortResult<ResolvedUser> {
        if issuer.trim().is_empty() || subject.trim().is_empty() {
            return Err(InfrastructureError::persistence(
                "resolve external identity",
                DatabaseError::invalid_value("external identity", "empty issuer or subject"),
            ));
        }
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin identity resolution", e))?;
        if let Some(existing) = identity::Entity::find()
            .filter(identity::Column::Issuer.eq(issuer))
            .filter(identity::Column::Subject.eq(subject))
            .one(&tx)
            .await
            .map_err(|e| InfrastructureError::database("load external identity", e))?
        {
            let user_id = UserId::try_from(existing.user_id).map_err(ApplicationPortError::from)?;
            tx.commit()
                .await
                .map_err(|e| InfrastructureError::database("commit identity lookup", e))?;
            return Ok(ResolvedUser {
                user_id,
                is_new: false,
            });
        }
        let user_id =
            UserId::try_from(Uuid::new_v4().to_string()).map_err(ApplicationPortError::from)?;
        user::ActiveModel::for_external_identity(&user_id, subject)
            .insert(&tx)
            .await
            .map_err(|e| InfrastructureError::database("create user", e))?;
        let result = identity::Entity::insert(identity::ActiveModel::for_external_identity(
            issuer, subject, &user_id,
        ))
        .on_conflict(
            OnConflict::columns([identity::Column::Issuer, identity::Column::Subject])
                .do_nothing()
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await
        .map_err(|e| InfrastructureError::database("create external identity", e))?;
        match result {
            0 => {
                user::Entity::delete_by_id(user_id.as_str().to_owned())
                    .exec(&tx)
                    .await
                    .map_err(|e| InfrastructureError::database("discard duplicate user", e))?;
                let existing = identity::Entity::find()
                    .filter(identity::Column::Issuer.eq(issuer))
                    .filter(identity::Column::Subject.eq(subject))
                    .one(&tx)
                    .await
                    .map_err(|e| InfrastructureError::database("resolve concurrent identity", e))?
                    .ok_or_else(|| {
                        InfrastructureError::persistence(
                            "resolve concurrent identity",
                            DatabaseError::expected_record_missing("external identity"),
                        )
                    })?;
                let user_id =
                    UserId::try_from(existing.user_id).map_err(ApplicationPortError::from)?;
                tx.commit()
                    .await
                    .map_err(|e| InfrastructureError::database("commit identity resolution", e))?;
                Ok(ResolvedUser {
                    user_id,
                    is_new: false,
                })
            }
            _ => {
                tx.commit()
                    .await
                    .map_err(|e| InfrastructureError::database("commit user provisioning", e))?;
                Ok(ResolvedUser {
                    user_id,
                    is_new: true,
                })
            }
        }
    }
}

#[async_trait]
impl UserWriter for UserRepository {
    async fn delete(&self, user_id: &UserId) -> PortResult<()> {
        user::Entity::delete_by_id(user_id.as_str().to_owned())
            .exec(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("delete user and owned data", e))?;
        Ok(())
    }
}
