use crate::shared::application::PortResult;
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, QueryOrder, TransactionTrait,
};
use std::sync::Arc;

use super::super::error::InfrastructureError;
use super::entities::{preferences as pref, recent_application};
use crate::{
    catalog::domain::ApplicationId,
    preferences::{
        application::ports::{PreferencesReader, PreferencesWriter},
        domain::UserPreferences,
    },
    shared::domain::UserId,
};

#[derive(Clone, Debug)]
pub struct PreferencesRepository {
    connection: Arc<DatabaseConnection>,
}

impl PreferencesRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }

    pub(crate) async fn save_in(
        &self,
        tx: &DatabaseTransaction,
        value: &UserPreferences,
    ) -> PortResult<()> {
        let model = pref::ActiveModel::from(value);
        pref::Entity::insert(model)
            .on_conflict(
                OnConflict::column(pref::Column::OwnerId)
                    .update_columns([
                        pref::Column::LightThemeId,
                        pref::Column::DarkThemeId,
                        pref::Column::Language,
                        pref::Column::TimeZone,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await
            .map_err(|e| InfrastructureError::database("save preferences", e))?;
        recent_application::Entity::delete_many()
            .filter(recent_application::Column::OwnerId.eq(value.user_id().as_str()))
            .exec(tx)
            .await
            .map_err(|e| InfrastructureError::database("replace recent applications", e))?;
        for (position, id) in value.recently_accessed_application_ids().iter().enumerate() {
            recent_application::ActiveModel::from_preferences(value, id.as_uuid(), position as i32)
                .insert(tx)
                .await
                .map_err(|e| InfrastructureError::database("save recent application", e))?;
        }
        Ok(())
    }
}

#[async_trait]
impl PreferencesReader for PreferencesRepository {
    async fn load(&self, owner: &UserId) -> PortResult<Option<UserPreferences>> {
        let Some(row) = pref::Entity::find_by_id(owner.as_str().to_owned())
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load preferences", e))?
        else {
            return Ok(None);
        };
        let recent = recent_application::Entity::find()
            .filter(recent_application::Column::OwnerId.eq(owner.as_str()))
            .order_by_asc(recent_application::Column::Position)
            .all(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load recent applications", e))?;
        let ids = recent
            .into_iter()
            .map(|row| ApplicationId::from(row.application_id))
            .collect();
        Ok(Some(row.into_domain(owner, ids)?))
    }
}

#[async_trait]
impl PreferencesWriter for PreferencesRepository {
    async fn save(&self, value: &UserPreferences) -> PortResult<()> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin preferences save", e))?;
        self.save_in(&tx, value).await?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit preferences", e))
    }
}
