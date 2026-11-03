use crate::{
    catalog::{
        application::{
            commands::{CreateApplication, CreateApplicationInput},
            ports::{ApplicationReader, ApplicationWriter},
        },
        domain::{ApplicationHealthCheck, ApplicationId, ExpectedStatusClass, HealthCheckUrl},
    },
    shared::application::{ApplicationError, ApplicationResult},
};

/// Handles `POST /api/v1/applications`.
pub async fn handle<R>(
    repository: R,
    request: &dash_api::models::CreateApplicationRequest,
) -> ApplicationResult<ApplicationId>
where
    R: ApplicationReader + ApplicationWriter,
{
    let health_check = request
        .health_check
        .as_ref()
        .and_then(|nullable| match nullable {
            dash_api::types::Nullable::Null => None,
            dash_api::types::Nullable::Present(check) => Some(check),
        })
        .map(|check| {
            let url = check
                .url
                .as_ref()
                .and_then(|url| match url {
                    dash_api::types::Nullable::Null => None,
                    dash_api::types::Nullable::Present(url) => Some(url.as_str()),
                })
                .map(HealthCheckUrl::try_from)
                .transpose()
                .map_err(|error| {
                    ApplicationError::from(crate::catalog::domain::CatalogError::from(error))
                })?;
            let status = match check.expected_status_class.as_deref().unwrap_or("2xx") {
                "2xx" => ExpectedStatusClass::Success2xx,
                "3xx" => ExpectedStatusClass::Redirect3xx,
                "4xx" => ExpectedStatusClass::ClientError4xx,
                other => return Err(invalid_value("health_check.expected_status_class", other)),
            };
            Ok::<_, ApplicationError>(ApplicationHealthCheck::new(
                url,
                status,
                check.ignore_tls_errors.unwrap_or(false),
            ))
        })
        .transpose()?;
    CreateApplication::new(repository)
        .execute(CreateApplicationInput {
            name: request.name.clone(),
            icon: request.icon.clone(),
            address: request.address.clone(),
            visible_to_groups: request.visible_to_groups.clone(),
            health_check,
        })
        .await
}

fn invalid_value(field: &'static str, value: &str) -> ApplicationError {
    ApplicationError::infrastructure(
        field,
        std::io::Error::new(std::io::ErrorKind::InvalidInput, value.to_owned()),
    )
}
