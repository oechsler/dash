use crate::shared::application::PortResult;
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, QueryOrder, TransactionTrait,
};
use std::collections::HashMap;
use std::sync::Arc;

use super::super::error::InfrastructureError;
use super::entities::{bookmark, category, dashboard};
use crate::{
    dashboard::{
        application::ports::{DashboardReader, DashboardWriter},
        domain::{Bookmark, UserDashboard},
    },
    shared::domain::UserId,
};

#[derive(Clone, Debug)]
pub struct DashboardRepository {
    connection: Arc<DatabaseConnection>,
}

impl DashboardRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }

    pub(crate) async fn save_in(
        &self,
        tx: &DatabaseTransaction,
        value: &UserDashboard,
    ) -> PortResult<()> {
        category::Entity::delete_many()
            .filter(category::Column::OwnerId.eq(value.owner().as_str()))
            .exec(tx)
            .await
            .map_err(|e| InfrastructureError::database("replace dashboard categories", e))?;
        let model = dashboard::ActiveModel::from(value);
        dashboard::Entity::insert(model)
            .on_conflict(
                OnConflict::column(dashboard::Column::OwnerId)
                    .update_column(dashboard::Column::CategoriesSortOrder)
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await
            .map_err(|e| InfrastructureError::database("save dashboard", e))?;
        for (position, item) in value.categories().iter().enumerate() {
            category::ActiveModel::from_domain(item, value.owner(), position as i32)
                .insert(tx)
                .await
                .map_err(|e| InfrastructureError::database("save dashboard category", e))?;
            for (bookmark_position, row) in item.bookmarks().iter().enumerate() {
                bookmark::ActiveModel::from_domain(
                    row,
                    item.id().as_uuid(),
                    bookmark_position as i32,
                )
                .insert(tx)
                .await
                .map_err(|e| InfrastructureError::database("save dashboard bookmark", e))?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl DashboardReader for DashboardRepository {
    async fn load(&self, owner: &UserId) -> PortResult<Option<UserDashboard>> {
        let Some(stored_dashboard) = dashboard::Entity::find_by_id(owner.as_str().to_owned())
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load dashboard", e))?
        else {
            return Ok(None);
        };
        let stored_categories = category::Entity::find()
            .filter(category::Column::OwnerId.eq(owner.as_str()))
            .order_by_asc(category::Column::Position)
            .all(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("load dashboard categories", e))?;
        let ids = stored_categories
            .iter()
            .map(|row| row.id)
            .collect::<Vec<_>>();
        let stored_bookmarks = if ids.is_empty() {
            Vec::new()
        } else {
            bookmark::Entity::find()
                .filter(bookmark::Column::CategoryId.is_in(ids))
                .order_by_asc(bookmark::Column::CategoryId)
                .order_by_asc(bookmark::Column::Position)
                .all(self.connection.as_ref())
                .await
                .map_err(|e| InfrastructureError::database("load dashboard bookmarks", e))?
        };
        let mut bookmarks_by_category: HashMap<uuid::Uuid, Vec<Bookmark>> = HashMap::new();
        for row in stored_bookmarks {
            bookmarks_by_category
                .entry(row.category_id)
                .or_default()
                .push(Bookmark::try_from(row)?);
        }
        let mut restored = Vec::with_capacity(stored_categories.len());
        for row in stored_categories {
            let bookmarks = bookmarks_by_category.remove(&row.id).unwrap_or_default();
            restored.push(row.into_domain(bookmarks)?);
        }
        Ok(Some(stored_dashboard.into_domain(restored)?))
    }
}

#[async_trait]
impl DashboardWriter for DashboardRepository {
    async fn save(&self, value: &UserDashboard) -> PortResult<()> {
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin dashboard save", e))?;
        self.save_in(&tx, value).await?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit dashboard save", e))
    }
}
