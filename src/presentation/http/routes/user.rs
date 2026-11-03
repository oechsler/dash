use crate::presentation::http::{
    api::{HttpApi, HttpClaims},
    error::HttpError,
    middleware::require_scope,
};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

pub fn router() -> Router<HttpApi> {
    Router::new().route("/me", get(get_current_user).delete(delete_current_user))
}
async fn get_current_user(
    State(_api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_user_read")?;
    let identity = claims.oidc_identity.as_ref().ok_or(
        crate::shared::application::ApplicationError::NotFound("verified identity"),
    )?;
    let body = crate::user::presentation::http::handlers::get_current_user_handler::handle(
        &claims.user_id,
        identity,
    );
    Ok((StatusCode::OK, axum::Json(body)).into_response())
}
async fn delete_current_user(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_user_delete")?;
    crate::user::presentation::http::handlers::delete_current_user_handler::handle(
        api.users,
        &claims.user_id,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
