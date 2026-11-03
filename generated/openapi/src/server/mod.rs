use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::{
    TypedHeader,
    extract::{CookieJar, Query as QueryExtra},
};
use bytes::Bytes;
use headers::Host;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, header::CONTENT_TYPE};
use tracing::error;
use validator::{Validate, ValidationErrors};

#[allow(unused_imports)]
use crate::{apis, models};
use crate::{header, types::*};
#[allow(unused_imports)]
use crate::{
    models::check_xss_map, models::check_xss_map_nested, models::check_xss_map_string,
    models::check_xss_string, models::check_xss_vec_string,
};

/// Setup API Server.
pub fn new<I, A, E, C>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::applications::Applications<E, Claims = C>
        + apis::dashboard::Dashboard<E, Claims = C>
        + apis::preferences::Preferences<E, Claims = C>
        + apis::sessions::Sessions<E, Claims = C>
        + apis::themes::Themes<E, Claims = C>
        + apis::transfer::Transfer<E, Claims = C>
        + apis::user::User<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync
        + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    // build our application with a route
    Router::new()
        .route(
            "/api/v1/applications",
            get(list_applications::<I, A, E, C>)
                .post(create_application::<I, A, E, C>)
                .put(update_application_catalog_sort_order::<I, A, E, C>),
        )
        .route(
            "/api/v1/applications/order",
            put(update_application_catalog_order::<I, A, E, C>),
        )
        .route(
            "/api/v1/applications/{application_id}",
            delete(delete_application::<I, A, E, C>)
                .get(get_application::<I, A, E, C>)
                .put(update_application::<I, A, E, C>),
        )
        .route(
            "/api/v1/applications/{application_id}/access",
            post(record_application_access::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories",
            post(create_category::<I, A, E, C>).put(update_categories_sort_order::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/order",
            put(reorder_categories::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/{category_id}",
            delete(delete_category::<I, A, E, C>).put(update_category::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/{category_id}/bookmarks",
            post(create_bookmark::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/{category_id}/bookmarks/order",
            put(reorder_bookmarks::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/{category_id}/bookmarks/{bookmark_id}",
            delete(delete_bookmark::<I, A, E, C>).put(update_bookmark::<I, A, E, C>),
        )
        .route(
            "/api/v1/categories/{category_id}/bookmarks/{bookmark_id}/access",
            post(record_bookmark_access::<I, A, E, C>),
        )
        .route("/api/v1/dashboard", get(get_dashboard::<I, A, E, C>))
        .route(
            "/api/v1/me",
            delete(delete_current_user::<I, A, E, C>).get(get_current_user::<I, A, E, C>),
        )
        .route(
            "/api/v1/preferences",
            get(get_preferences::<I, A, E, C>).put(update_preferences::<I, A, E, C>),
        )
        .route("/api/v1/sessions", get(list_sessions::<I, A, E, C>))
        .route("/api/v1/sessions/login", get(begin_login::<I, A, E, C>))
        .route(
            "/api/v1/sessions/login/callback",
            get(complete_login::<I, A, E, C>),
        )
        .route("/api/v1/sessions/logout", post(logout::<I, A, E, C>))
        .route(
            "/api/v1/sessions/{session_id}",
            delete(invalidate_session::<I, A, E, C>),
        )
        .route(
            "/api/v1/sessions/{session_id}/downgrade",
            post(downgrade_session::<I, A, E, C>),
        )
        .route(
            "/api/v1/sessions/{session_id}/terminate",
            post(terminate_session::<I, A, E, C>),
        )
        .route(
            "/api/v1/sessions/{session_id}/upgrade",
            post(upgrade_session::<I, A, E, C>),
        )
        .route(
            "/api/v1/themes",
            get(list_themes::<I, A, E, C>).post(create_theme::<I, A, E, C>),
        )
        .route(
            "/api/v1/themes/{theme_id}",
            delete(delete_theme::<I, A, E, C>).get(get_theme::<I, A, E, C>),
        )
        .route(
            "/api/v1/transfer/export",
            get(export_transfer::<I, A, E, C>),
        )
        .route(
            "/api/v1/transfer/import",
            post(import_transfer::<I, A, E, C>),
        )
        .with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateApplicationBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::CreateApplicationRequest,
}

#[tracing::instrument(skip_all)]
fn create_application_validation(
    body: models::CreateApplicationRequest,
) -> std::result::Result<(models::CreateApplicationRequest,), ValidationErrors> {
    let b = CreateApplicationBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// CreateApplication - POST /api/v1/applications
#[tracing::instrument(skip_all)]
async fn create_application<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::CreateApplicationRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || create_application_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .create_application(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::CreateApplicationResponse::Status201_TheSharedAppHasBeenAdded
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::CreateApplicationResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::CreateApplicationResponse::Status403_YouDon
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(403);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::CreateApplicationResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_application_validation(
    path_params: models::DeleteApplicationPathParams,
) -> std::result::Result<(models::DeleteApplicationPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteApplication - DELETE /api/v1/applications/{application_id}
#[tracing::instrument(skip_all)]
async fn delete_application<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::DeleteApplicationPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || delete_application_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_application(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::DeleteApplicationResponse::Status204_TheAppHasBeenRemovedFromTheSharedCatalog
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::applications::DeleteApplicationResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::DeleteApplicationResponse::Status403_YouDon
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(403);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::DeleteApplicationResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_application_validation(
    path_params: models::GetApplicationPathParams,
) -> std::result::Result<(models::GetApplicationPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetApplication - GET /api/v1/applications/{application_id}
#[tracing::instrument(skip_all)]
async fn get_application<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::GetApplicationPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_application_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_application(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::applications::GetApplicationResponse::Status200_TheSharedApp(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::applications::GetApplicationResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::applications::GetApplicationResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn list_applications_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ListApplications - GET /api/v1/applications
#[tracing::instrument(skip_all)]
async fn list_applications<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || list_applications_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .list_applications(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::applications::ListApplicationsResponse::Status200_AllAppsInTheSharedCatalog(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::applications::ListApplicationsResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::applications::ListApplicationsResponse::Status403_YouDon(body) => {
                let mut response = Response::builder();
                let mut response = response.status(403);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn record_application_access_validation(
    path_params: models::RecordApplicationAccessPathParams,
) -> std::result::Result<(models::RecordApplicationAccessPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// RecordApplicationAccess - POST /api/v1/applications/{application_id}/access
#[tracing::instrument(skip_all)]
async fn record_application_access<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::RecordApplicationAccessPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || record_application_access_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .record_application_access(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::RecordApplicationAccessResponse::Status204_TheAppHasBeenAddedToYourRecent
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::applications::RecordApplicationAccessResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::RecordApplicationAccessResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateApplicationBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdateApplicationRequest,
}

#[tracing::instrument(skip_all)]
fn update_application_validation(
    path_params: models::UpdateApplicationPathParams,
    body: models::UpdateApplicationRequest,
) -> std::result::Result<
    (
        models::UpdateApplicationPathParams,
        models::UpdateApplicationRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = UpdateApplicationBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateApplication - PUT /api/v1/applications/{application_id}
#[tracing::instrument(skip_all)]
async fn update_application<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::UpdateApplicationPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdateApplicationRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_application_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_application(&method, &host, &cookies, &claims, &path_params, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::UpdateApplicationResponse::Status200_TheSharedAppHasBeenUpdated
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationResponse::Status403_YouDon
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(403);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateApplicationCatalogOrderBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdateApplicationCatalogOrderRequest,
}

#[tracing::instrument(skip_all)]
fn update_application_catalog_order_validation(
    body: models::UpdateApplicationCatalogOrderRequest,
) -> std::result::Result<(models::UpdateApplicationCatalogOrderRequest,), ValidationErrors> {
    let b = UpdateApplicationCatalogOrderBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// UpdateApplicationCatalogOrder - PUT /api/v1/applications/order
#[tracing::instrument(skip_all)]
async fn update_application_catalog_order<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdateApplicationCatalogOrderRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_application_catalog_order_validation(body))
            .await
            .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_application_catalog_order(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::UpdateApplicationCatalogOrderResponse::Status204_YourCustomAppOrderHasBeenSaved
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::applications::UpdateApplicationCatalogOrderResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationCatalogOrderResponse::Status403_YouDon
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(403);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationCatalogOrderResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateApplicationCatalogSortOrderBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdateCategoriesSortOrderRequest,
}

#[tracing::instrument(skip_all)]
fn update_application_catalog_sort_order_validation(
    body: models::UpdateCategoriesSortOrderRequest,
) -> std::result::Result<(models::UpdateCategoriesSortOrderRequest,), ValidationErrors> {
    let b = UpdateApplicationCatalogSortOrderBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// UpdateApplicationCatalogSortOrder - PUT /api/v1/applications
#[tracing::instrument(skip_all)]
async fn update_application_catalog_sort_order<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdateCategoriesSortOrderRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::applications::Applications<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_application_catalog_sort_order_validation(body))
            .await
            .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_application_catalog_sort_order(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::applications::UpdateApplicationCatalogSortOrderResponse::Status204_TheAppCatalogSortingRuleHasBeenSaved
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::applications::UpdateApplicationCatalogSortOrderResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationCatalogSortOrderResponse::Status403_YouDon
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(403);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::applications::UpdateApplicationCatalogSortOrderResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateBookmarkBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::CreateBookmarkRequest,
}

#[tracing::instrument(skip_all)]
fn create_bookmark_validation(
    path_params: models::CreateBookmarkPathParams,
    body: models::CreateBookmarkRequest,
) -> std::result::Result<
    (
        models::CreateBookmarkPathParams,
        models::CreateBookmarkRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = CreateBookmarkBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// CreateBookmark - POST /api/v1/categories/{category_id}/bookmarks
#[tracing::instrument(skip_all)]
async fn create_bookmark<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::CreateBookmarkPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::CreateBookmarkRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || create_bookmark_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .create_bookmark(&method, &host, &cookies, &claims, &path_params, &body)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::CreateBookmarkResponse::Status201_YourLinkHasBeenSaved(body) => {
                let mut response = Response::builder();
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateBookmarkResponse::Status400_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateBookmarkResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateBookmarkResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateBookmarkResponse::Status422_OneOrMoreValuesNeedToBeCorrected(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(422);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateCategoryBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::CreateCategoryRequest,
}

#[tracing::instrument(skip_all)]
fn create_category_validation(
    body: models::CreateCategoryRequest,
) -> std::result::Result<(models::CreateCategoryRequest,), ValidationErrors> {
    let b = CreateCategoryBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// CreateCategory - POST /api/v1/categories
#[tracing::instrument(skip_all)]
async fn create_category<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::CreateCategoryRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || create_category_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .create_category(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::CreateCategoryResponse::Status201_YourNewCategoryIsReadyToUse(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateCategoryResponse::Status400_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateCategoryResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::CreateCategoryResponse::Status422_OneOrMoreValuesNeedToBeCorrected(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(422);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_bookmark_validation(
    path_params: models::DeleteBookmarkPathParams,
) -> std::result::Result<(models::DeleteBookmarkPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteBookmark - DELETE /api/v1/categories/{category_id}/bookmarks/{bookmark_id}
#[tracing::instrument(skip_all)]
async fn delete_bookmark<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::DeleteBookmarkPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_bookmark_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_bookmark(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::DeleteBookmarkResponse::Status204_YourLinkHasBeenRemoved => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::dashboard::DeleteBookmarkResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::DeleteBookmarkResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_category_validation(
    path_params: models::DeleteCategoryPathParams,
) -> std::result::Result<(models::DeleteCategoryPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteCategory - DELETE /api/v1/categories/{category_id}
#[tracing::instrument(skip_all)]
async fn delete_category<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::DeleteCategoryPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_category_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_category(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::dashboard::DeleteCategoryResponse::Status204_TheCategoryAndItsBookmarksHaveBeenDeleted
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::dashboard::DeleteCategoryResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::DeleteCategoryResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_dashboard_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetDashboard - GET /api/v1/dashboard
#[tracing::instrument(skip_all)]
async fn get_dashboard<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_dashboard_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_dashboard(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::GetDashboardResponse::Status200_YourApps(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::GetDashboardResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn record_bookmark_access_validation(
    path_params: models::RecordBookmarkAccessPathParams,
) -> std::result::Result<(models::RecordBookmarkAccessPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// RecordBookmarkAccess - POST /api/v1/categories/{category_id}/bookmarks/{bookmark_id}/access
#[tracing::instrument(skip_all)]
async fn record_bookmark_access<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::RecordBookmarkAccessPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || record_bookmark_access_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .record_bookmark_access(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::dashboard::RecordBookmarkAccessResponse::Status204_YourBookmarkHasBeenMarkedAsOpened
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::dashboard::RecordBookmarkAccessResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::RecordBookmarkAccessResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::RecordBookmarkAccessResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ReorderBookmarksBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::ReorderBookmarksRequest,
}

#[tracing::instrument(skip_all)]
fn reorder_bookmarks_validation(
    path_params: models::ReorderBookmarksPathParams,
    body: models::ReorderBookmarksRequest,
) -> std::result::Result<
    (
        models::ReorderBookmarksPathParams,
        models::ReorderBookmarksRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = ReorderBookmarksBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// ReorderBookmarks - PUT /api/v1/categories/{category_id}/bookmarks/order
#[tracing::instrument(skip_all)]
async fn reorder_bookmarks<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::ReorderBookmarksPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::ReorderBookmarksRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || reorder_bookmarks_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .reorder_bookmarks(&method, &host, &cookies, &claims, &path_params, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::dashboard::ReorderBookmarksResponse::Status204_TheLinkOrderHasBeenSaved
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::dashboard::ReorderBookmarksResponse::Status400_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::ReorderBookmarksResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::ReorderBookmarksResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::ReorderBookmarksResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ReorderCategoriesBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::ReorderCategoriesRequest,
}

#[tracing::instrument(skip_all)]
fn reorder_categories_validation(
    body: models::ReorderCategoriesRequest,
) -> std::result::Result<(models::ReorderCategoriesRequest,), ValidationErrors> {
    let b = ReorderCategoriesBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// ReorderCategories - PUT /api/v1/categories/order
#[tracing::instrument(skip_all)]
async fn reorder_categories<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::ReorderCategoriesRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || reorder_categories_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .reorder_categories(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::dashboard::ReorderCategoriesResponse::Status204_YourCategoryOrderHasBeenSaved
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::dashboard::ReorderCategoriesResponse::Status400_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::ReorderCategoriesResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::ReorderCategoriesResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateBookmarkBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::CreateBookmarkRequest,
}

#[tracing::instrument(skip_all)]
fn update_bookmark_validation(
    path_params: models::UpdateBookmarkPathParams,
    body: models::CreateBookmarkRequest,
) -> std::result::Result<
    (
        models::UpdateBookmarkPathParams,
        models::CreateBookmarkRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = UpdateBookmarkBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateBookmark - PUT /api/v1/categories/{category_id}/bookmarks/{bookmark_id}
#[tracing::instrument(skip_all)]
async fn update_bookmark<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::UpdateBookmarkPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::CreateBookmarkRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_bookmark_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_bookmark(&method, &host, &cookies, &claims, &path_params, &body)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::UpdateBookmarkResponse::Status200_YourLinkHasBeenUpdated(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateBookmarkResponse::Status400_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateBookmarkResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateBookmarkResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateCategoriesSortOrderBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdateCategoriesSortOrderRequest,
}

#[tracing::instrument(skip_all)]
fn update_categories_sort_order_validation(
    body: models::UpdateCategoriesSortOrderRequest,
) -> std::result::Result<(models::UpdateCategoriesSortOrderRequest,), ValidationErrors> {
    let b = UpdateCategoriesSortOrderBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// UpdateCategoriesSortOrder - PUT /api/v1/categories
#[tracing::instrument(skip_all)]
async fn update_categories_sort_order<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdateCategoriesSortOrderRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_categories_sort_order_validation(body))
            .await
            .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_categories_sort_order(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::dashboard::UpdateCategoriesSortOrderResponse::Status204_YourCategorySortingRuleHasBeenSaved
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::dashboard::UpdateCategoriesSortOrderResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::dashboard::UpdateCategoriesSortOrderResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdateCategoryBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdateCategoryRequest,
}

#[tracing::instrument(skip_all)]
fn update_category_validation(
    path_params: models::UpdateCategoryPathParams,
    body: models::UpdateCategoryRequest,
) -> std::result::Result<
    (
        models::UpdateCategoryPathParams,
        models::UpdateCategoryRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = UpdateCategoryBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// UpdateCategory - PUT /api/v1/categories/{category_id}
#[tracing::instrument(skip_all)]
async fn update_category<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::UpdateCategoryPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdateCategoryRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::dashboard::Dashboard<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || update_category_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_category(&method, &host, &cookies, &claims, &path_params, &body)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::dashboard::UpdateCategoryResponse::Status200_YourCategoryHasBeenUpdated(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateCategoryResponse::Status400_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateCategoryResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateCategoryResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::dashboard::UpdateCategoryResponse::Status422_OneOrMoreValuesNeedToBeCorrected(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(422);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_preferences_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetPreferences - GET /api/v1/preferences
#[tracing::instrument(skip_all)]
async fn get_preferences<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::preferences::Preferences<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_preferences_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_preferences(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::preferences::GetPreferencesResponse::Status200_YourLanguage(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::preferences::GetPreferencesResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UpdatePreferencesBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UpdatePreferencesRequest,
}

#[tracing::instrument(skip_all)]
fn update_preferences_validation(
    body: models::UpdatePreferencesRequest,
) -> std::result::Result<(models::UpdatePreferencesRequest,), ValidationErrors> {
    let b = UpdatePreferencesBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// UpdatePreferences - PUT /api/v1/preferences
#[tracing::instrument(skip_all)]
async fn update_preferences<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::UpdatePreferencesRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::preferences::Preferences<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || update_preferences_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .update_preferences(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::preferences::UpdatePreferencesResponse::Status200_YourPreferencesHaveBeenSaved
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::preferences::UpdatePreferencesResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::preferences::UpdatePreferencesResponse::Status404_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(404);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::preferences::UpdatePreferencesResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn begin_login_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// BeginLogin - GET /api/v1/sessions/login
#[tracing::instrument(skip_all)]
async fn begin_login<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || begin_login_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .begin_login(&method, &host, &cookies)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::sessions::BeginLoginResponse::Status302_ContinueToYourAccountProviderToSignIn
                                                    {
                                                        location
                                                    }
                                                => {
                                                let mut response = Response::builder();
                    if let Some(location) = location {
                        let location = match header::IntoHeaderValue(location).try_into() {
                            Ok(val) => val,
                            Err(e) => {
                                return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling location header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                            }
                        };

                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                              HeaderName::from_static("location"),
                              location
                        );
                    }
                                                  let mut response = response.status(302);
                                                  response.body(Body::empty())
                                                },
                                                apis::sessions::BeginLoginResponse::Status500_SomethingWentWrongWhileHandlingTheRequest
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(500);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn complete_login_validation(
    query_params: models::CompleteLoginQueryParams,
) -> std::result::Result<(models::CompleteLoginQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// CompleteLogin - GET /api/v1/sessions/login/callback
#[tracing::instrument(skip_all)]
async fn complete_login<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    QueryExtra(query_params): QueryExtra<models::CompleteLoginQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || complete_login_validation(query_params))
        .await
        .unwrap();

    let Ok((query_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .complete_login(&method, &host, &cookies, &query_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::CompleteLoginResponse::Status302_RedirectsBackToTheAppAfterSign {
                location,
                set_cookie,
            } => {
                let mut response = Response::builder();
                if let Some(location) = location {
                    let location = match header::IntoHeaderValue(location).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling location header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                        }
                    };

                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(HeaderName::from_static("location"), location);
                }
                if let Some(set_cookie) = set_cookie {
                    let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling set_cookie header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                        }
                    };

                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(HeaderName::from_static("set-cookie"), set_cookie);
                }
                let mut response = response.status(302);
                response.body(Body::empty())
            }
            apis::sessions::CompleteLoginResponse::Status400_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::CompleteLoginResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn downgrade_session_validation(
    path_params: models::DowngradeSessionPathParams,
) -> std::result::Result<(models::DowngradeSessionPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DowngradeSession - POST /api/v1/sessions/{session_id}/downgrade
#[tracing::instrument(skip_all)]
async fn downgrade_session<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::DowngradeSessionPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || downgrade_session_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .downgrade_session(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::DowngradeSessionResponse::Status204_ThisDeviceWillUseAShort => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::sessions::DowngradeSessionResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::DowngradeSessionResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::DowngradeSessionResponse::Status409_ThisDeviceIsAlreadyUsingAShort(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(409);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn invalidate_session_validation(
    path_params: models::InvalidateSessionPathParams,
) -> std::result::Result<(models::InvalidateSessionPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// InvalidateSession - DELETE /api/v1/sessions/{session_id}
#[tracing::instrument(skip_all)]
async fn invalidate_session<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::InvalidateSessionPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || invalidate_session_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .invalidate_session(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::InvalidateSessionResponse::Status204_ThatDeviceHasBeenSignedOut => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::sessions::InvalidateSessionResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::InvalidateSessionResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn list_sessions_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ListSessions - GET /api/v1/sessions
#[tracing::instrument(skip_all)]
async fn list_sessions<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || list_sessions_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .list_sessions(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::ListSessionsResponse::Status200_YourSigned(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::ListSessionsResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn logout_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// Logout - POST /api/v1/sessions/logout
#[tracing::instrument(skip_all)]
async fn logout<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims = None.or(claims_in_cookie);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || logout_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .logout(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::sessions::LogoutResponse::Status303_RedirectToTheAccountProviderToCompleteSign
                                                    {
                                                        location,
                                                        set_cookie
                                                    }
                                                => {
                                                let mut response = Response::builder();
                    if let Some(location) = location {
                        let location = match header::IntoHeaderValue(location).try_into() {
                            Ok(val) => val,
                            Err(e) => {
                                return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling location header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                            }
                        };

                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                              HeaderName::from_static("location"),
                              location
                        );
                    }
                    if let Some(set_cookie) = set_cookie {
                        let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
                            Ok(val) => val,
                            Err(e) => {
                                return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling set_cookie header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                            }
                        };

                        let mut response_headers = response.headers_mut().unwrap();
                        response_headers.insert(
                              HeaderName::from_static("set-cookie"),
                              set_cookie
                        );
                    }
                                                  let mut response = response.status(303);
                                                  response.body(Body::empty())
                                                },
                                                apis::sessions::LogoutResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn terminate_session_validation(
    path_params: models::TerminateSessionPathParams,
) -> std::result::Result<(models::TerminateSessionPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// TerminateSession - POST /api/v1/sessions/{session_id}/terminate
#[tracing::instrument(skip_all)]
async fn terminate_session<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::TerminateSessionPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || terminate_session_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .terminate_session(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::TerminateSessionResponse::Status204_ThatDeviceHasBeenSignedOut => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::sessions::TerminateSessionResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::TerminateSessionResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn upgrade_session_validation(
    path_params: models::UpgradeSessionPathParams,
) -> std::result::Result<(models::UpgradeSessionPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// UpgradeSession - POST /api/v1/sessions/{session_id}/upgrade
#[tracing::instrument(skip_all)]
async fn upgrade_session<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::UpgradeSessionPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::sessions::Sessions<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || upgrade_session_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .upgrade_session(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::sessions::UpgradeSessionResponse::Status204_ThisDeviceWillStaySignedInLonger => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::sessions::UpgradeSessionResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::UpgradeSessionResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::sessions::UpgradeSessionResponse::Status409_ThisDeviceIsAlreadyUsingALong(
                body,
            ) => {
                let mut response = Response::builder();
                let mut response = response.status(409);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct CreateThemeBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::CreateThemeRequest,
}

#[tracing::instrument(skip_all)]
fn create_theme_validation(
    body: models::CreateThemeRequest,
) -> std::result::Result<(models::CreateThemeRequest,), ValidationErrors> {
    let b = CreateThemeBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// CreateTheme - POST /api/v1/themes
#[tracing::instrument(skip_all)]
async fn create_theme<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::CreateThemeRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::themes::Themes<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || create_theme_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .create_theme(&method, &host, &cookies, &claims, &body)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::themes::CreateThemeResponse::Status201_YourNewThemeIsReadyToUse(body) => {
                let mut response = Response::builder();
                let mut response = response.status(201);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::CreateThemeResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::CreateThemeResponse::Status422_OneOrMoreValuesNeedToBeCorrected(body) => {
                let mut response = Response::builder();
                let mut response = response.status(422);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_theme_validation(
    path_params: models::DeleteThemePathParams,
) -> std::result::Result<(models::DeleteThemePathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// DeleteTheme - DELETE /api/v1/themes/{theme_id}
#[tracing::instrument(skip_all)]
async fn delete_theme<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::DeleteThemePathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::themes::Themes<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_theme_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_theme(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::themes::DeleteThemeResponse::Status204_YourThemeHasBeenDeleted => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::themes::DeleteThemeResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::DeleteThemeResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::DeleteThemeResponse::Status409_YouCan(body) => {
                let mut response = Response::builder();
                let mut response = response.status(409);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_theme_validation(
    path_params: models::GetThemePathParams,
) -> std::result::Result<(models::GetThemePathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetTheme - GET /api/v1/themes/{theme_id}
#[tracing::instrument(skip_all)]
async fn get_theme<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    Path(path_params): Path<models::GetThemePathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::themes::Themes<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_theme_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_theme(&method, &host, &cookies, &claims, &path_params)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::themes::GetThemeResponse::Status200_TheTheme(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::GetThemeResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::GetThemeResponse::Status404_WeCouldn(body) => {
                let mut response = Response::builder();
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn list_themes_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ListThemes - GET /api/v1/themes
#[tracing::instrument(skip_all)]
async fn list_themes<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::themes::Themes<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || list_themes_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .list_themes(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::themes::ListThemesResponse::Status200_Built(body) => {
                let mut response = Response::builder();
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::themes::ListThemesResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn export_transfer_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ExportTransfer - GET /api/v1/transfer/export
#[tracing::instrument(skip_all)]
async fn export_transfer<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::transfer::Transfer<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || export_transfer_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .export_transfer(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::transfer::ExportTransferResponse::Status200_YourJSONBackupIsReadyToDownload {
                body,
                content_disposition,
            } => {
                let mut response = Response::builder();
                if let Some(content_disposition) = content_disposition {
                    let content_disposition = match header::IntoHeaderValue(content_disposition)
                        .try_into()
                    {
                        Ok(val) => val,
                        Err(e) => {
                            return Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::from(format!("An internal server error occurred handling content_disposition header - {e}"))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                        }
                    };

                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        HeaderName::from_static("content-disposition"),
                        content_disposition,
                    );
                }
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers
                        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::transfer::ExportTransferResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn import_transfer_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ImportTransfer - POST /api/v1/transfer/import
#[tracing::instrument(skip_all)]
async fn import_transfer<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    body: Multipart,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::transfer::Transfer<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || import_transfer_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .import_transfer(&method, &host, &cookies, &claims, body)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::transfer::ImportTransferResponse::Status204_YourDashboardHasBeenRestoredFromTheBackup
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::transfer::ImportTransferResponse::Status400_WeCouldn
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::transfer::ImportTransferResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::transfer::ImportTransferResponse::Status422_OneOrMoreValuesNeedToBeCorrected
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(422);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn delete_current_user_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// DeleteCurrentUser - DELETE /api/v1/me
#[tracing::instrument(skip_all)]
async fn delete_current_user<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || delete_current_user_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .delete_current_user(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::user::DeleteCurrentUserResponse::Status204_YourDataHasBeenDeleted => {
                let mut response = Response::builder();
                let mut response = response.status(204);
                response.body(Body::empty())
            }
            apis::user::DeleteCurrentUserResponse::Status401_SignInToContinue(body) => {
                let mut response = Response::builder();
                let mut response = response.status(401);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/problem+json"),
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_current_user_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetCurrentUser - GET /api/v1/me
#[tracing::instrument(skip_all)]
async fn get_current_user<I, A, E, C>(
    method: Method,
    TypedHeader(host): TypedHeader<Host>,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::user::User<E, Claims = C>
        + apis::CookieAuthentication<Claims = C>
        + apis::ApiAuthBasic<Claims = C>
        + Send
        + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // Authentication
    let claims_in_cookie = api_impl
        .as_ref()
        .extract_claims_from_cookie(&cookies, "dash_session")
        .await;
    let claims_in_auth_header = api_impl
        .as_ref()
        .extract_claims_from_auth_header(apis::BasicAuthKind::Bearer, &headers, "authorization")
        .await;
    let claims = None.or(claims_in_cookie).or(claims_in_auth_header);
    let Some(claims) = claims else {
        return response_with_status_code_only(StatusCode::UNAUTHORIZED);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || get_current_user_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_current_user(&method, &host, &cookies, &claims)
        .await;

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::user::GetCurrentUserResponse::Status200_YourProfileAndTheGroupsThatGiveYouAccessToSharedApps
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::user::GetCurrentUserResponse::Status401_SignInToContinue
                                                    (body)
                                                => {
                                                let mut response = Response::builder();
                                                  let mut response = response.status(401);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_static("application/problem+json"));
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(why) => {
                                                    // Application code returned an error. This should not happen, as the implementation should
                                                    // return a valid response.
                                                    return api_impl.as_ref().handle_error(&method, &host, &cookies, why).await;
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[allow(dead_code)]
#[inline]
fn response_with_status_code_only(code: StatusCode) -> Result<Response, StatusCode> {
    Response::builder()
        .status(code)
        .body(Body::empty())
        .map_err(|_| code)
}
