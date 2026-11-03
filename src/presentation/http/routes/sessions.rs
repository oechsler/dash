use crate::{
    presentation::http::{
        api::{HttpApi, HttpClaims},
        error::HttpError,
        middleware::require_scope,
    },
    session::{
        application::commands::{CreateSession, CreateSessionInput},
        domain::{SessionId, SessionMetadata},
    },
};
use axum::{
    Router,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;
use uuid::Uuid;

pub fn router() -> Router<HttpApi> {
    Router::new()
        .route("/sessions", get(list_sessions))
        .route("/sessions/login", get(begin_login))
        .route("/sessions/login/callback", get(complete_login))
        .route("/sessions/logout", post(logout))
        .route(
            "/sessions/{session_id}",
            axum::routing::delete(invalidate_session),
        )
        .route("/sessions/{session_id}/downgrade", post(downgrade_session))
        .route("/sessions/{session_id}/upgrade", post(upgrade_session))
        .route("/sessions/{session_id}/terminate", post(terminate_session))
}
#[derive(Deserialize)]
struct Callback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}
async fn begin_login(State(api): State<HttpApi>) -> Result<Redirect, HttpError> {
    let url = crate::session::presentation::http::handlers::begin_login_handler::handle(&api.oidc)
        .map_err(|e| {
            crate::shared::application::ApplicationError::infrastructure("begin OIDC login", e)
        })?;
    Ok(Redirect::temporary(&url))
}
async fn complete_login(
    State(api): State<HttpApi>,
    Query(query): Query<Callback>,
) -> Result<Response, HttpError> {
    if query.error.is_some() {
        tracing::warn!(description=?query.error_description,"OIDC provider rejected login");
        return Err(crate::shared::application::ApplicationError::BadRequest.into());
    }
    let code = query
        .code
        .as_deref()
        .ok_or(crate::shared::application::ApplicationError::BadRequest)?;
    let state = query
        .state
        .as_deref()
        .ok_or(crate::shared::application::ApplicationError::BadRequest)?;
    let identity = crate::session::presentation::http::handlers::complete_login_handler::handle(
        &api.oidc, code, state,
    )
    .await
    .map_err(|e| {
        tracing::warn!(error=%e,"OIDC callback rejected");
        crate::shared::application::ApplicationError::BadRequest
    })?;
    let external = crate::user::application::commands::ExternalIdentity {
        issuer: identity.issuer.clone(),
        subject: identity.subject.clone(),
    };
    let (user_id, _) =
        crate::user::application::commands::ResolveOrCreateUser::new(api.users.clone())
            .execute(&external)
            .await?;
    let now = chrono::Utc::now();
    let expires = now + chrono::Duration::hours(1);
    let id = CreateSession::new(api.sessions.clone())
        .execute(CreateSessionInput {
            owner: user_id.clone(),
            metadata: SessionMetadata::default(),
            issued_at: now,
            expires_at: expires,
        })
        .await?;
    let token = api
        .signed_session_cookie(id.as_uuid(), &user_id, &identity, expires)
        .ok_or_else(|| {
            crate::shared::application::ApplicationError::infrastructure(
                "sign session cookie",
                std::io::Error::other("session signing unavailable"),
            )
        })?;
    let cookie =
        format!("dash_session={token}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=3600");
    let mut response = (StatusCode::FOUND, Redirect::temporary("/")).into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        cookie.parse().map_err(|e| {
            crate::shared::application::ApplicationError::infrastructure("set session cookie", e)
        })?,
    );
    Ok(response)
}
async fn list_sessions(
    State(api): State<HttpApi>,
    claims: HttpClaims,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_read")?;
    let now = chrono::Utc::now();
    let sessions = crate::session::presentation::http::handlers::list_sessions_handler::handle(
        api.sessions,
        &claims.user_id,
    )
    .await?;
    let result: Vec<_> = sessions
        .into_iter()
        .map(|s| {
            let mut metadata = dash_api::models::ListSessions200ResponseInnerMetadata::new();
            metadata.browser = s
                .metadata()
                .browser()
                .map(|v| dash_api::types::Nullable::Present(v.to_owned()));
            metadata.operating_system = s
                .metadata()
                .operating_system()
                .map(|v| dash_api::types::Nullable::Present(v.to_owned()));
            dash_api::models::ListSessions200ResponseInner::new(
                s.id().as_uuid(),
                s.issued_at(),
                s.last_accessed_at(),
                s.expires_at(),
                s.is_active_at(now),
                Some(s.id().as_uuid()) == claims.current_session,
                if s.is_long_lived() { "long" } else { "short" }.to_owned(),
                metadata,
            )
        })
        .collect();
    Ok((StatusCode::OK, axum::Json(result)).into_response())
}
fn id(value: Uuid) -> SessionId {
    value.into()
}
async fn invalidate_session(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(session): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_write")?;
    crate::session::presentation::http::handlers::invalidate_session_handler::handle(
        api.sessions,
        &claims.user_id,
        id(session),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn terminate_session(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(session): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_write")?;
    crate::session::presentation::http::handlers::terminate_session_handler::handle(
        api.sessions,
        &claims.user_id,
        id(session),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn upgrade_session(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(session): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_write")?;
    crate::session::presentation::http::handlers::upgrade_session_handler::handle(
        api.sessions,
        &claims.user_id,
        id(session),
        chrono::Duration::days(30),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn downgrade_session(
    State(api): State<HttpApi>,
    claims: HttpClaims,
    Path(session): Path<Uuid>,
) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_write")?;
    crate::session::presentation::http::handlers::downgrade_session_handler::handle(
        api.sessions,
        &claims.user_id,
        id(session),
        chrono::Duration::hours(1),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
async fn logout(State(api): State<HttpApi>, claims: HttpClaims) -> Result<Response, HttpError> {
    require_scope(&claims, "dash_sessions_write")?;
    let location = api
        .oidc
        .end_session_url()
        .map_err(|e| {
            crate::shared::application::ApplicationError::infrastructure("build OIDC logout URL", e)
        })?
        .ok_or_else(|| {
            crate::shared::application::ApplicationError::infrastructure(
                "build OIDC logout URL",
                std::io::Error::other("provider has no end-session endpoint"),
            )
        })?;
    if let Some(session) = claims.current_session {
        crate::session::presentation::http::handlers::logout_handler::handle(
            api.sessions,
            &claims.user_id,
            id(session),
        )
        .await?;
    }
    let mut response = (StatusCode::SEE_OTHER, Redirect::to(&location)).into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        "dash_session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0"
            .parse()
            .expect("static cookie header"),
    );
    Ok(response)
}
