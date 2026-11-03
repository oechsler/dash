use sea_orm::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use crate::{
    catalog::domain::{ApplicationCatalogSettings, ApplicationId},
    shared::{
        application::{ApplicationPortError, PortResult},
        domain::CollectionSortOrder,
        infrastructure::{DatabaseAdapterError as InfrastructureError, DatabaseError},
    },
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "application_catalog_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub singleton: bool,
    pub sort_order: String,
    pub application_order: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_domain(self) -> PortResult<ApplicationCatalogSettings> {
        let mut settings = ApplicationCatalogSettings::default();
        let sort_order = CollectionSortOrder::try_from(self.sort_order.as_str()).map_err(|_| {
            InfrastructureError::persistence(
                "decode catalog sort order",
                DatabaseError::invalid_value("sort_order", self.sort_order.clone()),
            )
        })?;
        settings.set_sort_order(sort_order);
        let ids = self
            .application_order
            .split(',')
            .filter(|id| !id.is_empty())
            .map(|id| {
                Uuid::parse_str(id)
                    .map(ApplicationId::from)
                    .map_err(|error| {
                        InfrastructureError::database("decode application order", error)
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if !ids.is_empty() {
            settings
                .set_manual_order(ids.clone(), &ids)
                .map_err(ApplicationPortError::from)?;
        }
        Ok(settings)
    }
}

impl ActiveModel {
    pub fn from_domain(value: &ApplicationCatalogSettings) -> Self {
        Self {
            singleton: Set(true),
            sort_order: Set(value.sort_order().as_str().to_owned()),
            application_order: Set(value
                .manual_order()
                .iter()
                .map(|id| id.as_uuid().to_string())
                .collect::<Vec<_>>()
                .join(",")),
        }
    }
}

impl ActiveModel {
    pub fn without_application(current: Model, removed: ApplicationId) -> Self {
        let removed = removed.as_uuid().to_string();
        let application_order = current
            .application_order
            .split(',')
            .filter(|key| *key != removed)
            .collect::<Vec<_>>()
            .join(",");
        Self {
            singleton: Set(true),
            sort_order: Set(current.sort_order),
            application_order: Set(application_order),
        }
    }
}
