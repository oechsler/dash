use std::{collections::HashSet, sync::Arc};

use axum_extra::extract::CookieJar;
use hmac::{Hmac, KeyInit, Mac};
use http::HeaderMap;
use sea_orm::DatabaseConnection;
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    catalog::infrastructure::sea_orm::CatalogRepository,
    dashboard::infrastructure::sea_orm::DashboardRepository,
    preferences::infrastructure::sea_orm::PreferencesRepository,
    session::{application::ports::SessionReader, infrastructure::sea_orm::SessionRepository},
    shared::domain::{GroupId, UserId},
    theme::infrastructure::sea_orm::ThemeRepository,
    transfer::infrastructure::sea_orm::UserDataSnapshotRepository,
    user::{
        application::commands::{ExternalIdentity, ResolveOrCreateUser},
        infrastructure::sea_orm::UserRepository,
    },
};

use super::auth::{OidcIdentity, OidcProvider};

/// Verified identity and authorization data attached by the HTTP auth extractor.
#[derive(Clone, Debug)]
pub struct HttpClaims {
    pub user_id: UserId,
    pub groups: HashSet<GroupId>,
    pub scopes: HashSet<String>,
    pub is_administrator: bool,
    pub current_session: Option<Uuid>,
    pub oidc_identity: Option<OidcIdentity>,
    pub authenticated_by_session: bool,
}

impl HttpClaims {
    fn from_oidc(user_id: UserId, identity: OidcIdentity) -> Self {
        let groups = identity
            .groups
            .iter()
            .filter_map(|group| GroupId::try_from(group.as_str()).ok())
            .collect();
        Self {
            user_id,
            groups,
            scopes: identity.scopes.iter().cloned().collect(),
            is_administrator: identity.is_administrator,
            current_session: None,
            oidc_identity: Some(identity),
            authenticated_by_session: false,
        }
    }

    pub fn allows(&self, scope: &str) -> bool {
        self.scopes.contains(scope)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SessionCookieClaims {
    session_id: Uuid,
    user_id: String,
    groups: Vec<String>,
    scopes: Vec<String>,
    is_administrator: bool,
    identity: OidcIdentity,
    expires_at: chrono::DateTime<chrono::Utc>,
}

/// HTTP application state shared by the feature routers and auth extractor.
#[derive(Clone)]
pub struct HttpApi {
    pub(crate) catalog: CatalogRepository,
    pub(crate) dashboard: DashboardRepository,
    pub(crate) preferences: PreferencesRepository,
    pub(crate) sessions: SessionRepository,
    pub(crate) themes: ThemeRepository,
    pub(crate) transfer: UserDataSnapshotRepository,
    pub(crate) users: UserRepository,
    pub(crate) oidc: Arc<OidcProvider>,
    session_signing_key: Arc<Vec<u8>>,
}

impl HttpApi {
    pub fn new(
        connection: Arc<DatabaseConnection>,
        oidc: Arc<OidcProvider>,
        session_signing_key: Vec<u8>,
    ) -> Result<Self, &'static str> {
        if session_signing_key.len() < 32 {
            return Err("session signing key must contain at least 32 bytes");
        }
        Ok(Self {
            catalog: CatalogRepository::new(Arc::clone(&connection)),
            dashboard: DashboardRepository::new(Arc::clone(&connection)),
            preferences: PreferencesRepository::new(Arc::clone(&connection)),
            sessions: SessionRepository::new(Arc::clone(&connection)),
            themes: ThemeRepository::new(Arc::clone(&connection)),
            transfer: UserDataSnapshotRepository::new(Arc::clone(&connection)),
            users: UserRepository::new(connection),
            oidc,
            session_signing_key: Arc::new(session_signing_key),
        })
    }

    pub fn router(self) -> axum::Router {
        super::routes::router(self)
    }
}

impl HttpApi {
    pub(crate) fn signed_session_cookie(
        &self,
        id: Uuid,
        user_id: &UserId,
        identity: &OidcIdentity,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Option<String> {
        let key = self.session_signing_key.as_ref();
        let claims = SessionCookieClaims {
            session_id: id,
            user_id: user_id.as_str().to_owned(),
            groups: identity.groups.clone(),
            scopes: identity.scopes.clone(),
            is_administrator: identity.is_administrator,
            identity: identity.clone(),
            expires_at,
        };
        let payload = encode_hex(&serde_json::to_vec(&claims).ok()?);
        let mut mac = Hmac::<Sha256>::new_from_slice(key).ok()?;
        mac.update(payload.as_bytes());
        Some(format!(
            "{payload}.{}",
            encode_hex(&mac.finalize().into_bytes())
        ))
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let digit = |ch: u8| match ch {
                b'0'..=b'9' => Some(ch - b'0'),
                b'a'..=b'f' => Some(ch - b'a' + 10),
                b'A'..=b'F' => Some(ch - b'A' + 10),
                _ => None,
            };
            Some(digit(pair[0])? * 16 + digit(pair[1])?)
        })
        .collect()
}

impl HttpApi {
    pub(crate) async fn claims_from_cookie(
        &self,
        cookies: &CookieJar,
        key: &str,
    ) -> Option<HttpClaims> {
        let signing_key = self.session_signing_key.as_ref();
        let mut parts = cookies.get(key)?.value().splitn(3, '.');
        let payload = parts.next()?;
        let signature = parts.next()?;
        if parts.next().is_some() {
            return None;
        }
        let signature = decode_hex(signature)?;
        let mut mac = Hmac::<Sha256>::new_from_slice(signing_key).ok()?;
        mac.update(payload.as_bytes());
        mac.verify_slice(&signature).ok()?;
        let cookie: SessionCookieClaims = serde_json::from_slice(&decode_hex(payload)?).ok()?;
        if cookie.expires_at <= chrono::Utc::now() {
            return None;
        }
        let session = self
            .sessions
            .load_by_id(cookie.session_id.into())
            .await
            .ok()??;
        if !session.is_active_at(chrono::Utc::now()) {
            return None;
        }
        if session.owner().as_str() != cookie.user_id {
            return None;
        }
        Some(HttpClaims {
            user_id: session.owner().clone(),
            groups: cookie
                .groups
                .iter()
                .filter_map(|g| GroupId::try_from(g.as_str()).ok())
                .collect(),
            scopes: cookie.scopes.into_iter().collect(),
            is_administrator: cookie.is_administrator,
            current_session: Some(cookie.session_id),
            oidc_identity: Some(cookie.identity),
            authenticated_by_session: true,
        })
    }

    pub(crate) async fn claims_from_bearer(
        &self,
        headers: &HeaderMap,
        key: &str,
    ) -> Option<HttpClaims> {
        let header = headers.get(key)?.to_str().ok()?;
        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))?;
        let identity = self.oidc.validate_bearer(token).await.ok()?;
        let external_identity = ExternalIdentity {
            issuer: identity.issuer.clone(),
            subject: identity.subject.clone(),
        };
        let (user_id, _) = ResolveOrCreateUser::new(self.users.clone())
            .execute(&external_identity)
            .await
            .ok()?;
        Some(HttpClaims::from_oidc(user_id, identity))
    }
}
