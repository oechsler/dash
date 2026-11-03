use axum::{extract::FromRequestParts, http::request::Parts, middleware::Next, response::Response};
use std::time::Instant;

use super::{
    api::{HttpApi, HttpClaims},
    error::HttpError,
};

/// Request-scoped identity extractor. Credentials are read only at this boundary.
impl FromRequestParts<HttpApi> for HttpClaims {
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &HttpApi,
    ) -> Result<Self, Self::Rejection> {
        let cookie_jar =
            <axum_extra::extract::CookieJar as FromRequestParts<HttpApi>>::from_request_parts(
                parts, state,
            )
            .await
            .map_err(|_| HttpError(crate::shared::application::ApplicationError::BadRequest))?;
        if let Some(claims) = state.claims_from_cookie(&cookie_jar, "dash_session").await {
            return Ok(claims);
        }
        if let Some(claims) = state
            .claims_from_bearer(&parts.headers, "authorization")
            .await
        {
            return Ok(claims);
        }
        Err(HttpError(
            crate::shared::application::ApplicationError::Unauthorized,
        ))
    }
}

pub fn require_scope(claims: &HttpClaims, scope: &str) -> Result<(), HttpError> {
    if claims.allows(scope) {
        Ok(())
    } else {
        Err(HttpError(
            crate::shared::application::ApplicationError::Forbidden,
        ))
    }
}

/// Logs each request after the response, matching the previous Fiber app's
/// application-wide request logging responsibility.
pub async fn request_log(request: axum::http::Request<axum::body::Body>, next: Next) -> Response {
    let started = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let user_agent = request
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("-")
        .to_owned();
    let response = next.run(request).await;
    tracing::info!(
        status = response.status().as_u16(),
        %method,
        %path,
        elapsed_ms = started.elapsed().as_millis(),
        %user_agent,
        "HTTP request"
    );
    response
}
