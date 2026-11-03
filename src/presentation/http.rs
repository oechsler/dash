//! Schema-generated Axum API boundary.
//!
//! The OpenAPI document is the source of truth for routes, request and response
//! types, and generated endpoint contracts. The generated router is an Axum
//! Tower service and can be composed into a larger Axum or Hyper server.
//! Application-facing endpoint adapters belong in per-operation handler files.

pub use dash_api::server::new as generated_router;
pub use dash_api::{apis, models};

pub mod api;
mod api_applications;
mod api_dashboard;
mod api_preferences;
mod api_sessions;
mod api_themes;
mod api_transfer;
pub mod auth;

pub use api::{HttpApi, HttpClaims};

/// Creates the schema-generated API router wired to the application adapters.
pub fn router(api: HttpApi) -> axum::Router {
    api.router()
}
