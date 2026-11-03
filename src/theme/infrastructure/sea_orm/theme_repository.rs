use super::super::error::InfrastructureError;
use super::entities::{ActiveModel, Column, Entity};
use crate::shared::application::PortResult;
use crate::{
    shared::application::ApplicationPortError,
    shared::domain::UserId,
    theme::{
        application::ports::{ThemeReader, ThemeWriter},
        domain::{Theme, ThemeId},
    },
};
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder,
    TransactionTrait,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ThemeRepository {
    connection: Arc<DatabaseConnection>,
}

impl ThemeRepository {
    pub fn new(connection: impl Into<Arc<DatabaseConnection>>) -> Self {
        Self {
            connection: connection.into(),
        }
    }

    pub(crate) async fn save_in(&self, tx: &DatabaseTransaction, theme: &Theme) -> PortResult<()> {
        let model = ActiveModel::try_from_domain(theme)?;
        Entity::insert(model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([
                        Column::OwnerId,
                        Column::Name,
                        Column::Mode,
                        Column::PrimaryColor,
                        Column::SecondaryColor,
                        Column::TertiaryColor,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(tx)
            .await
            .map_err(|e| InfrastructureError::database("save theme", e))?;
        Ok(())
    }
}

#[async_trait]
impl ThemeReader for ThemeRepository {
    async fn list_for(&self, owner: &UserId) -> PortResult<Vec<Theme>> {
        let rows = Entity::find()
            .filter(Column::OwnerId.eq(owner.as_str()))
            .order_by_asc(Column::Name)
            .all(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("list themes", e))?;
        rows.into_iter().map(|row| row.into_domain(owner)).collect()
    }

    async fn get_for(&self, owner: &UserId, id: ThemeId) -> PortResult<Option<Theme>> {
        let ThemeId::UserDefined(id) = id else {
            return Ok(None);
        };
        let row = Entity::find_by_id(id.to_string())
            .filter(Column::OwnerId.eq(owner.as_str()))
            .one(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("get theme", e))?;
        row.map(|row| row.into_domain(owner)).transpose()
    }
}

#[async_trait]
impl ThemeWriter for ThemeRepository {
    async fn save(&self, theme: &Theme) -> PortResult<()> {
        let ThemeId::UserDefined(_) = theme.id() else {
            return Err(ApplicationPortError::NotFound("user-defined theme"));
        };
        let tx = self
            .connection
            .begin()
            .await
            .map_err(|e| InfrastructureError::database("begin theme save", e))?;

        self.save_in(&tx, theme).await?;
        tx.commit()
            .await
            .map_err(|e| InfrastructureError::database("commit theme save", e))
    }

    async fn delete(&self, owner: &UserId, id: ThemeId) -> PortResult<()> {
        let ThemeId::UserDefined(id) = id else {
            return Err(ApplicationPortError::NotFound("user-defined theme"));
        };
        Entity::delete_many()
            .filter(Column::Id.eq(id.to_string()))
            .filter(Column::OwnerId.eq(owner.as_str()))
            .exec(self.connection.as_ref())
            .await
            .map_err(|e| InfrastructureError::database("delete theme", e))?;
        Ok(())
    }
}
