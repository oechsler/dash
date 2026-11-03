use super::api::HttpApi;
use axum::Router;

mod applications;
mod dashboard;
mod preferences;
mod sessions;
mod themes;
mod transfer;
mod user;

/// The single route-registration point, analogous to the previous Fiber `RegisterAll`.
pub fn router(state: HttpApi) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .merge(applications::router())
                .merge(dashboard::router())
                .merge(preferences::router())
                .merge(sessions::router())
                .merge(themes::router())
                .merge(transfer::router())
                .merge(user::router()),
        )
        .with_state(state)
}
