use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use dash_api::{apis::applications as api, models, types::Nullable};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    catalog::domain::Application,
    shared::application::{ApplicationError, ApplicationResult},
};

fn scope(claims: &HttpClaims, name: &str) -> ApplicationResult<()> {
    if claims.allows(name) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}

fn response(app: Application) -> models::CreateApplication201Response {
    let visible_to_groups = app
        .visibility()
        .visible_to_groups()
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.as_str().to_owned())
                .collect()
        })
        .unwrap_or_default();
    let mut body = models::CreateApplication201Response::new(
        app.id().as_uuid(),
        app.name().to_owned(),
        app.icon().as_str().to_owned(),
        app.address().as_str().to_owned(),
        health_check(app.health_check()),
        visible_to_groups,
    );
    body.health = app.health().map(|health| match health {
        crate::catalog::domain::ApplicationHealth::Up => "up".to_owned(),
        crate::catalog::domain::ApplicationHealth::Down => "down".to_owned(),
    });
    body
}

fn health_check(
    check: Option<&crate::catalog::domain::ApplicationHealthCheck>,
) -> Nullable<models::ListApplications200ResponseApplicationsInnerHealthCheck> {
    let Some(check) = check else {
        return Nullable::Null;
    };
    let mut body = models::ListApplications200ResponseApplicationsInnerHealthCheck::new();
    body.url = Some(match check.url() {
        Some(url) => Nullable::Present(url.as_str().to_owned()),
        None => Nullable::Null,
    });
    body.expected_status_class = Some(check.expected_status_class().as_str().to_owned());
    body.ignore_tls_errors = Some(check.ignore_tls_errors());
    Nullable::Present(body)
}

#[async_trait]
impl api::Applications<ApplicationError> for HttpApi {
    type Claims = HttpClaims;

    async fn create_application(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        body: &models::CreateApplicationRequest,
    ) -> ApplicationResult<api::CreateApplicationResponse> {
        scope(claims, "dash_applications_write")?;
        if !claims.is_administrator {
            return Err(ApplicationError::Forbidden);
        }
        let id = crate::catalog::presentation::http::handlers::create_application_handler::handle(
            self.catalog.clone(),
            body,
        )
        .await?;
        let app = crate::catalog::presentation::http::handlers::get_application_handler::handle(
            self.catalog.clone(),
            id,
        )
        .await?;
        Ok(api::CreateApplicationResponse::Status201_TheSharedAppHasBeenAdded(response(app)))
    }

    async fn delete_application(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        path: &models::DeleteApplicationPathParams,
    ) -> ApplicationResult<api::DeleteApplicationResponse> {
        scope(claims, "dash_applications_write")?;
        if !claims.is_administrator {
            return Err(ApplicationError::Forbidden);
        }
        crate::catalog::presentation::http::handlers::delete_application_handler::handle(
            self.catalog.clone(),
            path.application_id.into(),
        )
        .await?;
        Ok(api::DeleteApplicationResponse::Status204_TheAppHasBeenRemovedFromTheSharedCatalog)
    }

    async fn get_application(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        path: &models::GetApplicationPathParams,
    ) -> ApplicationResult<api::GetApplicationResponse> {
        scope(claims, "dash_applications_read")?;
        let app = crate::catalog::presentation::http::handlers::get_application_handler::handle(
            self.catalog.clone(),
            path.application_id.into(),
        )
        .await?;
        if !claims.is_administrator && !app.is_visible_to(&claims.groups) {
            return Err(ApplicationError::NotFound("application"));
        }
        Ok(api::GetApplicationResponse::Status200_TheSharedApp(
            response(app),
        ))
    }

    async fn list_applications(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
    ) -> ApplicationResult<api::ListApplicationsResponse> {
        scope(claims, "dash_applications_read")?;
        let view = crate::catalog::presentation::http::handlers::list_applications_handler::handle(
            self.catalog.clone(),
            self.catalog.clone(),
            self.preferences.clone(),
            &claims.user_id,
        )
        .await?;
        let applications = view
            .applications
            .into_iter()
            .filter(|app| claims.is_administrator || app.is_visible_to(&claims.groups))
            .map(|app| {
                let visible = app
                    .visibility()
                    .visible_to_groups()
                    .map(|g| g.iter().map(|x| x.as_str().to_owned()).collect())
                    .unwrap_or_default();
                let mut item = models::ListApplications200ResponseApplicationsInner::new(
                    app.id().as_uuid(),
                    app.name().to_owned(),
                    app.icon().as_str().to_owned(),
                    app.address().as_str().to_owned(),
                    health_check(app.health_check()),
                    visible,
                );
                item.health = app.health().map(|h| {
                    match h {
                        crate::catalog::domain::ApplicationHealth::Up => "up",
                        crate::catalog::domain::ApplicationHealth::Down => "down",
                    }
                    .to_owned()
                });
                item
            })
            .collect();
        let body = models::ListApplications200Response::new(
            view.applications_sort_order.as_str().to_owned(),
            applications,
        );
        Ok(api::ListApplicationsResponse::Status200_AllAppsInTheSharedCatalog(body))
    }

    async fn record_application_access(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        path: &models::RecordApplicationAccessPathParams,
    ) -> ApplicationResult<api::RecordApplicationAccessResponse> {
        scope(claims, "dash_applications_read")?;
        crate::catalog::presentation::http::handlers::record_application_access_handler::handle(
            self.catalog.clone(),
            self.preferences.clone(),
            &claims.user_id,
            &claims.groups,
            path.application_id.into(),
        )
        .await?;
        Ok(api::RecordApplicationAccessResponse::Status204_TheAppHasBeenAddedToYourRecent)
    }

    async fn update_application(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        path: &models::UpdateApplicationPathParams,
        body: &models::UpdateApplicationRequest,
    ) -> ApplicationResult<api::UpdateApplicationResponse> {
        scope(claims, "dash_applications_write")?;
        if !claims.is_administrator {
            return Err(ApplicationError::Forbidden);
        }
        crate::catalog::presentation::http::handlers::update_application_handler::handle(
            self.catalog.clone(),
            path.application_id.into(),
            body,
        )
        .await?;
        let app = crate::catalog::presentation::http::handlers::get_application_handler::handle(
            self.catalog.clone(),
            path.application_id.into(),
        )
        .await?;
        Ok(api::UpdateApplicationResponse::Status200_TheSharedAppHasBeenUpdated(response(app)))
    }

    async fn update_application_catalog_order(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        body: &models::UpdateApplicationCatalogOrderRequest,
    ) -> ApplicationResult<api::UpdateApplicationCatalogOrderResponse> {
        scope(claims, "dash_applications_write")?;
        if !claims.is_administrator {
            return Err(ApplicationError::Forbidden);
        }
        crate::catalog::presentation::http::handlers::update_application_catalog_order_handler::handle(self.catalog.clone(), self.catalog.clone(), body).await?;
        Ok(api::UpdateApplicationCatalogOrderResponse::Status204_YourCustomAppOrderHasBeenSaved)
    }

    async fn update_application_catalog_sort_order(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        claims: &HttpClaims,
        body: &models::UpdateCategoriesSortOrderRequest,
    ) -> ApplicationResult<api::UpdateApplicationCatalogSortOrderResponse> {
        scope(claims, "dash_applications_write")?;
        if !claims.is_administrator {
            return Err(ApplicationError::Forbidden);
        }
        crate::catalog::presentation::http::handlers::update_application_catalog_sort_order_handler::handle(self.catalog.clone(), body).await?;
        Ok(api::UpdateApplicationCatalogSortOrderResponse::Status204_TheAppCatalogSortingRuleHasBeenSaved)
    }
}
