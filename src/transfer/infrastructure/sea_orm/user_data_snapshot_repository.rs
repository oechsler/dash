use async_trait::async_trait;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::sync::Arc;

use super::super::error::InfrastructureError;
use crate::{
    catalog::{
        application::ports::{ApplicationCatalogSettingsReader, ApplicationReader},
        infrastructure::sea_orm::CatalogRepository,
    },
    dashboard::{
        application::ports::DashboardReader, infrastructure::sea_orm::DashboardRepository,
    },
    preferences::{
        application::ports::PreferencesReader, infrastructure::sea_orm::PreferencesRepository,
    },
    shared::application::ApplicationPortError,
    shared::domain::UserId,
    theme::{application::ports::ThemeReader, infrastructure::sea_orm::ThemeRepository},
    transfer::application::{
        models::UserDataSnapshot,
        ports::{UserDataSnapshotReader, UserDataSnapshotWriter},
    },
};

/// Snapshot persistence is an adapter at the transfer boundary and composes
/// the feature repositories rather than duplicating their schema knowledge.
#[derive(Clone, Debug)]
pub struct UserDataSnapshotRepository {
    connection: Arc<DatabaseConnection>,
    dashboard: DashboardRepository,
    catalog: CatalogRepository,
    themes: ThemeRepository,
    preferences: PreferencesRepository,
}

impl UserDataSnapshotRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        let connection = connection.into();
        Self {
            connection: connection.clone(),
            dashboard: DashboardRepository::new(connection.clone()),
            catalog: CatalogRepository::new(connection.clone()),
            themes: ThemeRepository::new(connection.clone()),
            preferences: PreferencesRepository::new(connection),
        }
    }
}

#[async_trait]
impl UserDataSnapshotReader for UserDataSnapshotRepository {
    async fn load(&self, owner: &UserId) -> Result<UserDataSnapshot, ApplicationPortError> {
        Ok(UserDataSnapshot {
            dashboard: self.dashboard.load(owner).await?,
            preferences: self.preferences.load(owner).await?,
            themes: self.themes.list_for(owner).await?,
            applications: self.catalog.list().await?,
            application_catalog_settings: self.catalog.load().await?,
            imported_theme_names: None,
        })
    }
}

#[async_trait]
impl UserDataSnapshotWriter for UserDataSnapshotRepository {
    async fn save(
        &self,
        owner: &UserId,
        snapshot: &UserDataSnapshot,
    ) -> Result<(), ApplicationPortError> {
        if let Some(dashboard) = &snapshot.dashboard
            && dashboard.owner() != owner
        {
            return Err(InfrastructureError::owner_mismatch(
                "save snapshot dashboard",
                "dashboard",
            ));
        }
        if let Some(preferences) = &snapshot.preferences
            && preferences.user_id() != owner
        {
            return Err(InfrastructureError::owner_mismatch(
                "save snapshot preferences",
                "preferences",
            ));
        }
        for theme in &snapshot.themes {
            if !theme.identity().is_available_to(owner) || !theme.is_deletable() {
                return Err(InfrastructureError::invalid_theme_owner(
                    "save snapshot theme",
                ));
            }
        }

        let tx = self
            .connection
            .begin()
            .await
            .map_err(|error| InfrastructureError::database("begin snapshot save", error))?;
        for application in &snapshot.applications {
            self.catalog.save_application_in(&tx, application).await?;
        }
        if let Some(settings) = &snapshot.application_catalog_settings {
            self.catalog.save_settings_in(&tx, settings).await?;
        }
        if let Some(dashboard) = &snapshot.dashboard {
            self.dashboard.save_in(&tx, dashboard).await?;
        }
        for theme in &snapshot.themes {
            self.themes.save_in(&tx, theme).await?;
        }
        if let Some(preferences) = &snapshot.preferences {
            self.preferences.save_in(&tx, preferences).await?;
        }
        tx.commit()
            .await
            .map_err(|error| InfrastructureError::database("commit snapshot save", error))
    }
}
