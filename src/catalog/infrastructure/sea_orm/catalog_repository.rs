use super::super::error::InfrastructureError;
use super::entities::{application as app, settings, visibility_group};
use crate::{
    catalog::{
        application::ports::{
            ApplicationCatalogSettingsReader, ApplicationCatalogSettingsWriter, ApplicationReader,
            ApplicationWriter,
        },
        domain::{Application, ApplicationCatalogSettings, ApplicationId},
    },
    shared::application::ApplicationPortError,
};
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, QueryOrder, TransactionTrait,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CatalogRepository {
    connection: Arc<DatabaseConnection>,
}
impl CatalogRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }

    pub(crate) async fn save_application_in(
        &self,
        tx: &DatabaseTransaction,
        value: &Application,
    ) -> Result<(), ApplicationPortError> {
        visibility_group::Entity::delete_many()
            .filter(visibility_group::Column::ApplicationId.eq(value.id().as_uuid()))
            .exec(tx)
            .await
            .map_err(|e| InfrastructureError::database("replace application visibility", e))?;
        let active = app::ActiveModel::from(value);
        app::Entity::insert(active)
            .on_conflict(
                OnConflict::column(app::Column::Id)
                    .update_columns([
                        app::Column::Name,
                        app::Column::Icon,
                        app::Column::Address,
                        app::Column::HealthCheckUrl,
                        app::Column::HealthCheckStatusClass,
                        app::Column::HealthCheckIgnoreTlsErrors,
                        app::Column::HealthStatus,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await
            .map_err(|e| InfrastructureError::database("save catalog application", e))?;
        if let Some(groups) = value.visibility().visible_to_groups() {
            for group in groups {
                visibility_group::ActiveModel::for_application(value.id().as_uuid(), group)
                    .insert(tx)
                    .await
                    .map_err(|e| InfrastructureError::database("save application visibility", e))?;
            }
        }
        Ok(())
    }

    pub(crate) async fn save_settings_in(
        &self,
        tx: &DatabaseTransaction,
        value: &ApplicationCatalogSettings,
    ) -> Result<(), ApplicationPortError> {
        let model = settings::ActiveModel::from(value);
        settings::Entity::insert(model)
            .on_conflict(
                OnConflict::column(settings::Column::Singleton)
                    .update_columns([
                        settings::Column::SortOrder,
                        settings::Column::ApplicationOrder,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await
            .map_err(|e| InfrastructureError::database("save catalog settings", e))?;
        Ok(())
    }
}

#[async_trait]
impl ApplicationReader for CatalogRepository {
    async fn list(&self) -> Result<Vec<Application>, ApplicationPortError> {
        let rows = app::Entity::find()
            .order_by_asc(app::Column::Name)
            .all(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("list catalog applications", e))?;
        let application_ids = rows.iter().map(|row| row.id).collect::<Vec<_>>();
        let mut groups_by_application: HashMap<Uuid, Vec<_>> = HashMap::new();
        if !application_ids.is_empty() {
            let visibility_groups = visibility_group::Entity::find()
                .filter(visibility_group::Column::ApplicationId.is_in(application_ids))
                .all(self.connection.as_ref())
                .await
                .map_err(|e| InfrastructureError::database("load application visibility", e))?;
            for row in visibility_groups {
                groups_by_application
                    .entry(row.application_id)
                    .or_default()
                    .push(row.group_id()?);
            }
        }
        let mut result = Vec::with_capacity(rows.len());
        for row in rows {
            let groups = groups_by_application.remove(&row.id).unwrap_or_default();
            result.push(row.into_domain(groups)?);
        }
        Ok(result)
    }
    async fn get(&self, id: ApplicationId) -> Result<Option<Application>, ApplicationPortError> {
        let row = app::Entity::find_by_id(id.as_uuid())
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("get catalog application", e))?;
        match row {
            Some(row) => {
                let groups = visibility_group::Entity::find()
                    .filter(visibility_group::Column::ApplicationId.eq(row.id))
                    .all(self.connection.as_ref())
                    .await
                    .map_err(|e| InfrastructureError::database("load application visibility", e))?
                    .into_iter()
                    .map(visibility_group::Model::group_id)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(row.into_domain(groups)?))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl ApplicationWriter for CatalogRepository {
    async fn save(&self, value: &Application) -> Result<(), ApplicationPortError> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin application save", e))?;
        self.save_application_in(&tx, value).await?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit application save", e))
    }
    async fn delete(&self, id: ApplicationId) -> Result<(), ApplicationPortError> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin application delete", e))?;
        app::Entity::delete_by_id(id.as_uuid())
            .exec(&tx)
            .await
            .map_err(|e| InfrastructureError::database("delete catalog application", e))?;
        let current = settings::Entity::find_by_id(true)
            .one(&tx)
            .await
            .map_err(|e| InfrastructureError::database("load catalog settings for delete", e))?;
        if let Some(current) = current {
            settings::ActiveModel::without_application(current, id)
                .update(&tx)
                .await
                .map_err(|e| InfrastructureError::database("update catalog order", e))?;
        }
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit application delete", e))
    }
}

#[async_trait]
impl ApplicationCatalogSettingsReader for CatalogRepository {
    async fn load(&self) -> Result<Option<ApplicationCatalogSettings>, ApplicationPortError> {
        let Some(row) = settings::Entity::find_by_id(true)
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load catalog settings", e))?
        else {
            return Ok(None);
        };
        Ok(Some(ApplicationCatalogSettings::try_from(row)?))
    }
}

#[async_trait]
impl ApplicationCatalogSettingsWriter for CatalogRepository {
    async fn save(&self, value: &ApplicationCatalogSettings) -> Result<(), ApplicationPortError> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin catalog settings save", e))?;
        self.save_settings_in(&tx, value).await?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit catalog settings save", e))
    }
}
