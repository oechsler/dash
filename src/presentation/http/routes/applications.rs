use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, put},
};
use uuid::Uuid;

use crate::{
    catalog::{domain::Application, presentation::http::handlers},
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::{self, ApiJson, HttpError},
        middleware::require_scope,
    },
    shared::application::ApplicationError,
};

pub fn router() -> Router<HttpApi> {
    Router::new()
        .route(
            "/applications",
            get(list_applications)
                .post(create_application)
                .put(update_application_catalog_sort_order),
        )
        .route("/applications/order", put(update_application_catalog_order))
        .route(
            "/applications/{application_id}",
            get(get_application)
                .put(update_application)
                .delete(delete_application),
        )
        .route(
            "/applications/{application_id}/access",
            post(record_application_access),
        )
}

fn health_check(
    check: Option<&crate::catalog::domain::ApplicationHealthCheck>,
) -> dash_api::types::Nullable<
    dash_api::models::ListApplications200ResponseApplicationsInnerHealthCheck,
> {
    use dash_api::types::Nullable;
    let Some(check) = check else {
        return Nullable::Null;
    };
    let mut result =
        dash_api::models::ListApplications200ResponseApplicationsInnerHealthCheck::new();
    result.url = Some(check.url().map_or(Nullable::Null, |url| {
        Nullable::Present(url.as_str().to_owned())
    }));
    result.expected_status_class = Some(check.expected_status_class().as_str().to_owned());
    result.ignore_tls_errors = Some(check.ignore_tls_errors());
    Nullable::Present(result)
}

fn application_model(app: Application) -> dash_api::models::CreateApplication201Response {
    let groups = app
        .visibility()
        .visible_to_groups()
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.as_str().to_owned())
                .collect()
        })
        .unwrap_or_default();
    let mut model = dash_api::models::CreateApplication201Response::new(
        app.id().as_uuid(),
        app.name().to_owned(),
        app.icon().as_str().to_owned(),
        app.address().as_str().to_owned(),
        health_check(app.health_check()),
        groups,
    );
    model.health = app.health().map(|health| {
        match health {
            crate::catalog::domain::ApplicationHealth::Up => "up",
            crate::catalog::domain::ApplicationHealth::Down => "down",
        }
        .to_owned()
    });
    model
}

async fn list_applications(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_read")?;
    let view = handlers::list_applications_handler::handle(
        api.catalog.clone(),
        api.catalog.clone(),
        api.preferences.clone(),
        &claims.user_id,
    )
    .await?;
    let applications = view
        .applications
        .into_iter()
        .filter(|app| claims.is_administrator || app.is_visible_to(&claims.groups))
        .map(|app| {
            let groups = app
                .visibility()
                .visible_to_groups()
                .map(|set| set.iter().map(|group| group.as_str().to_owned()).collect())
                .unwrap_or_default();
            let mut model = dash_api::models::ListApplications200ResponseApplicationsInner::new(
                app.id().as_uuid(),
                app.name().to_owned(),
                app.icon().as_str().to_owned(),
                app.address().as_str().to_owned(),
                health_check(app.health_check()),
                groups,
            );
            model.health = app.health().map(|health| {
                match health {
                    crate::catalog::domain::ApplicationHealth::Up => "up",
                    crate::catalog::domain::ApplicationHealth::Down => "down",
                }
                .to_owned()
            });
            model
        })
        .collect();
    Ok(error::json(
        StatusCode::OK,
        dash_api::models::ListApplications200Response::new(
            view.applications_sort_order.as_str().to_owned(),
            applications,
        ),
    ))
}

async fn create_application(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::CreateApplicationRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_write")?;
    if !claims.is_administrator {
        return Err(ApplicationError::Forbidden.into());
    }
    let id = handlers::create_application_handler::handle(api.catalog.clone(), &body).await?;
    let app = handlers::get_application_handler::handle(api.catalog, id).await?;
    Ok(error::json(StatusCode::CREATED, application_model(app)))
}

async fn get_application(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_read")?;
    let app = handlers::get_application_handler::handle(api.catalog, id.into()).await?;
    if !claims.is_administrator && !app.is_visible_to(&claims.groups) {
        return Err(ApplicationError::NotFound("application").into());
    }
    Ok(error::json(StatusCode::OK, application_model(app)))
}

async fn update_application(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<Uuid>,
    ApiJson(body): ApiJson<dash_api::models::UpdateApplicationRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_write")?;
    if !claims.is_administrator {
        return Err(ApplicationError::Forbidden.into());
    }
    handlers::update_application_handler::handle(api.catalog.clone(), id.into(), &body).await?;
    let app = handlers::get_application_handler::handle(api.catalog, id.into()).await?;
    Ok(error::json(StatusCode::OK, application_model(app)))
}

async fn delete_application(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_write")?;
    if !claims.is_administrator {
        return Err(ApplicationError::Forbidden.into());
    }
    handlers::delete_application_handler::handle(api.catalog, id.into()).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn record_application_access(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(id): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_read")?;
    handlers::record_application_access_handler::handle(
        api.catalog,
        api.preferences,
        &claims.user_id,
        &claims.groups,
        id.into(),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn update_application_catalog_order(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::UpdateApplicationCatalogOrderRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_write")?;
    if !claims.is_administrator {
        return Err(ApplicationError::Forbidden.into());
    }
    handlers::update_application_catalog_order_handler::handle(
        api.catalog.clone(),
        api.catalog,
        &body,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn update_application_catalog_sort_order(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    ApiJson(body): ApiJson<dash_api::models::UpdateCategoriesSortOrderRequest>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_applications_write")?;
    if !claims.is_administrator {
        return Err(ApplicationError::Forbidden.into());
    }
    handlers::update_application_catalog_sort_order_handler::handle(api.catalog, &body).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
