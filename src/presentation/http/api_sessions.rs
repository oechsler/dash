use async_trait::async_trait;
use axum_extra::extract::CookieJar;
use dash_api::{apis::sessions as api, models};
use headers::Host;
use http::Method;

use super::api::{HttpApi, HttpClaims};
use crate::{
    session::{
        application::commands::{CreateSession, CreateSessionInput},
        domain::{SessionId, SessionMetadata},
    },
    shared::application::{ApplicationError, ApplicationResult},
};

fn check(c: &HttpClaims, required: &str) -> ApplicationResult<()> {
    if c.allows(required) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}
fn session_id(raw: uuid::Uuid) -> SessionId {
    raw.into()
}

#[async_trait]
impl api::Sessions<ApplicationError> for HttpApi {
    type Claims = HttpClaims;
    async fn begin_login(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
    ) -> ApplicationResult<api::BeginLoginResponse> {
        let location =
            crate::session::presentation::http::handlers::begin_login_handler::handle(&self.oidc)
                .map_err(|e| ApplicationError::infrastructure("begin OIDC login", e))?;
        Ok(
            api::BeginLoginResponse::Status302_ContinueToYourAccountProviderToSignIn {
                location: Some(location),
            },
        )
    }
    async fn complete_login(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        query: &models::CompleteLoginQueryParams,
    ) -> ApplicationResult<api::CompleteLoginResponse> {
        if query.error.is_some() {
            return Err(ApplicationError::BadRequest);
        }
        let code = query.code.as_deref().ok_or(ApplicationError::BadRequest)?;
        let state = query.state.as_deref().ok_or(ApplicationError::BadRequest)?;
        let identity =
            crate::session::presentation::http::handlers::complete_login_handler::handle(
                &self.oidc, code, state,
            )
            .await
            .map_err(|e| {
                tracing::warn!(error = %e, "OIDC callback rejected");
                ApplicationError::BadRequest
            })?;
        let external = crate::user::application::commands::ExternalIdentity {
            issuer: identity.issuer.clone(),
            subject: identity.subject.clone(),
        };
        let (user_id, _) =
            crate::user::application::commands::ResolveOrCreateUser::new(self.users.clone())
                .execute(&external)
                .await?;
        let now = chrono::Utc::now();
        let id = CreateSession::new(self.sessions.clone())
            .execute(CreateSessionInput {
                owner: user_id.clone(),
                metadata: SessionMetadata::default(),
                issued_at: now,
                expires_at: now + chrono::Duration::hours(1),
            })
            .await?;
        let expires_at = now + chrono::Duration::hours(1);
        let token =
            super::api::signed_session_cookie(self, id.as_uuid(), &user_id, &identity, expires_at)
                .ok_or_else(|| {
                    ApplicationError::infrastructure(
                        "sign session cookie",
                        std::io::Error::other(
                            "DASH_SESSION_SIGNING_KEY must be configured with at least 32 bytes",
                        ),
                    )
                })?;
        let cookie =
            format!("dash_session={token}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=3600");
        Ok(
            api::CompleteLoginResponse::Status302_RedirectsBackToTheAppAfterSign {
                location: Some("/".to_owned()),
                set_cookie: Some(cookie),
            },
        )
    }
    async fn downgrade_session(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::DowngradeSessionPathParams,
    ) -> ApplicationResult<api::DowngradeSessionResponse> {
        check(c, "dash_sessions_write")?;
        crate::session::presentation::http::handlers::downgrade_session_handler::handle(
            self.sessions.clone(),
            &c.user_id,
            session_id(p.session_id),
            chrono::Duration::hours(1),
        )
        .await?;
        Ok(api::DowngradeSessionResponse::Status204_ThisDeviceWillUseAShort)
    }
    async fn invalidate_session(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::InvalidateSessionPathParams,
    ) -> ApplicationResult<api::InvalidateSessionResponse> {
        check(c, "dash_sessions_write")?;
        crate::session::presentation::http::handlers::invalidate_session_handler::handle(
            self.sessions.clone(),
            &c.user_id,
            session_id(p.session_id),
        )
        .await?;
        Ok(api::InvalidateSessionResponse::Status204_ThatDeviceHasBeenSignedOut)
    }
    async fn list_sessions(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
    ) -> ApplicationResult<api::ListSessionsResponse> {
        check(c, "dash_sessions_read")?;
        let now = chrono::Utc::now();
        let sessions = crate::session::presentation::http::handlers::list_sessions_handler::handle(
            self.sessions.clone(),
            &c.user_id,
        )
        .await?;
        let values = sessions
            .into_iter()
            .map(|s| {
                let mut metadata = models::ListSessions200ResponseInnerMetadata::new();
                metadata.browser = s
                    .metadata()
                    .browser()
                    .map(|v| dash_api::types::Nullable::Present(v.to_owned()));
                metadata.operating_system = s
                    .metadata()
                    .operating_system()
                    .map(|v| dash_api::types::Nullable::Present(v.to_owned()));
                models::ListSessions200ResponseInner::new(
                    s.id().as_uuid(),
                    s.issued_at(),
                    s.last_accessed_at(),
                    s.expires_at(),
                    s.is_active_at(now),
                    Some(s.id().as_uuid()) == c.current_session,
                    if s.is_long_lived() { "long" } else { "short" }.to_owned(),
                    metadata,
                )
            })
            .collect();
        Ok(api::ListSessionsResponse::Status200_YourSigned(values))
    }
    async fn logout(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
    ) -> ApplicationResult<api::LogoutResponse> {
        check(c, "dash_sessions_write")?;
        let location = self
            .oidc
            .end_session_url()
            .map_err(|error| ApplicationError::infrastructure("build OIDC logout URL", error))?
            .ok_or_else(|| {
                ApplicationError::infrastructure(
                    "build OIDC logout URL",
                    std::io::Error::other("provider has no end_session_endpoint"),
                )
            })?;
        if let Some(id) = c.current_session {
            crate::session::presentation::http::handlers::logout_handler::handle(
                self.sessions.clone(),
                &c.user_id,
                session_id(id),
            )
            .await?;
        }
        Ok(
            api::LogoutResponse::Status303_RedirectToTheAccountProviderToCompleteSign {
                location: Some(location),
                set_cookie: Some(
                    "dash_session=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0".to_owned(),
                ),
            },
        )
    }
    async fn terminate_session(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::TerminateSessionPathParams,
    ) -> ApplicationResult<api::TerminateSessionResponse> {
        check(c, "dash_sessions_write")?;
        crate::session::presentation::http::handlers::terminate_session_handler::handle(
            self.sessions.clone(),
            &c.user_id,
            session_id(p.session_id),
        )
        .await?;
        Ok(api::TerminateSessionResponse::Status204_ThatDeviceHasBeenSignedOut)
    }
    async fn upgrade_session(
        &self,
        _: &Method,
        _: &Host,
        _: &CookieJar,
        c: &HttpClaims,
        p: &models::UpgradeSessionPathParams,
    ) -> ApplicationResult<api::UpgradeSessionResponse> {
        check(c, "dash_sessions_write")?;
        crate::session::presentation::http::handlers::upgrade_session_handler::handle(
            self.sessions.clone(),
            &c.user_id,
            session_id(p.session_id),
            chrono::Duration::days(30),
        )
        .await?;
        Ok(api::UpgradeSessionResponse::Status204_ThisDeviceWillStaySignedInLonger)
    }
}
