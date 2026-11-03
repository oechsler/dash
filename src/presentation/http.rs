//! Axum HTTP application. Routes and transport middleware are hand-written;
//! request/response data models are generated from the OpenAPI contract.

pub use dash_api::models;

pub mod api;
pub mod auth;
pub mod error;
pub mod middleware;
pub mod routes;

pub use api::{HttpApi, HttpClaims};

/// Builds the Axum application and mounts the feature routers.
pub fn router(api: HttpApi) -> axum::Router {
    routes::router(api).layer(axum::middleware::from_fn(middleware::request_log))
}
