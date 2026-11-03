use sea_orm::Set;
use sea_orm::entity::prelude::*;

use crate::{
    catalog::domain::{
        Application, ApplicationHealth, ApplicationHealthCheck, ApplicationId,
        ApplicationVisibility, CatalogError, ExpectedStatusClass, HealthCheckUrl,
    },
    icon::domain::IconReference,
    shared::domain::{GroupId, WebAddress},
    shared::{
        application::{ApplicationPortError, PortResult},
        infrastructure::{DatabaseAdapterError as InfrastructureError, DatabaseError},
    },
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "catalog_applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub health_check_url: Option<String>,
    pub health_check_status_class: Option<String>,
    pub health_check_ignore_tls_errors: bool,
    pub health_status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_domain(self, visible_to: Vec<GroupId>) -> PortResult<Application> {
        let check = match self.health_check_status_class.as_deref() {
            None => None,
            Some(class) => {
                let expected = match class {
                    "2xx" => ExpectedStatusClass::Success2xx,
                    "3xx" => ExpectedStatusClass::Redirect3xx,
                    "4xx" => ExpectedStatusClass::ClientError4xx,
                    _ => {
                        return Err(InfrastructureError::persistence(
                            "decode expected status",
                            DatabaseError::invalid_value("health_check_status_class", class),
                        ));
                    }
                };
                let url = self
                    .health_check_url
                    .map(HealthCheckUrl::try_from)
                    .transpose()
                    .map_err(|error| ApplicationPortError::from(CatalogError::from(error)))?;
                Some(ApplicationHealthCheck::new(
                    url,
                    expected,
                    self.health_check_ignore_tls_errors,
                ))
            }
        };
        let health = match self.health_status.as_deref() {
            Some("up") => Some(ApplicationHealth::Up),
            Some("down") => Some(ApplicationHealth::Down),
            Some(value) => {
                return Err(InfrastructureError::persistence(
                    "decode health state",
                    DatabaseError::invalid_value("health_status", value),
                ));
            }
            None => None,
        };
        Application::restore(
            ApplicationId::from(self.id),
            self.name,
            IconReference::try_from(self.icon).map_err(ApplicationPortError::from)?,
            WebAddress::try_from(self.address).map_err(ApplicationPortError::from)?,
            ApplicationVisibility::groups(visible_to),
            check,
            health,
        )
        .map_err(ApplicationPortError::from)
    }
}

impl ActiveModel {
    pub fn from_domain(value: &Application) -> Self {
        let (url, class, tls) = value
            .health_check()
            .map(|check| {
                (
                    check.url().map(|url| url.as_str().to_owned()),
                    Some(check.expected_status_class().as_str().to_owned()),
                    check.ignore_tls_errors(),
                )
            })
            .unwrap_or((None, None, false));
        let health = value.health().map(|health| match health {
            ApplicationHealth::Up => "up".to_owned(),
            ApplicationHealth::Down => "down".to_owned(),
        });
        Self {
            id: Set(value.id().as_uuid()),
            name: Set(value.name().to_owned()),
            icon: Set(value.icon().as_str().to_owned()),
            address: Set(value.address().as_str().to_owned()),
            health_check_url: Set(url),
            health_check_status_class: Set(class),
            health_check_ignore_tls_errors: Set(tls),
            health_status: Set(health),
        }
    }
}
