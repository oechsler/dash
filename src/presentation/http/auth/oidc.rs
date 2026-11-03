use std::{collections::HashMap, sync::Mutex, time::Instant};

use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IssuerUrl, LogoutRequest, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, PostLogoutRedirectUrl, ProviderMetadataWithLogout,
    RedirectUrl, Scope, TokenResponse,
    core::{CoreAuthenticationFlow, CoreClient},
};
use serde::Deserialize;
use thiserror::Error;

type DiscoveredClient = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

/// Configuration for an OpenID Connect relying party.
#[derive(Clone, Debug)]
pub struct OidcConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_url: String,
    pub requested_scopes: Vec<String>,
    pub groups_claim: String,
    pub scopes_claim: String,
    pub administrator_group: Option<String>,
    pub post_logout_redirect_url: Option<String>,
}

impl OidcConfig {
    /// Loads the provider configuration from the `DASH_OIDC_*` environment variables.
    pub fn from_env() -> Result<Self, OidcProviderError> {
        let required = |name: &str| {
            std::env::var(name).map_err(|_| {
                OidcProviderError::Configuration(format!("environment variable {name} is required"))
            })
        };
        let optional = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());
        let requested_scopes = optional("DASH_OIDC_SCOPES")
            .unwrap_or_else(|| "openid profile email".to_owned())
            .split_whitespace()
            .map(str::to_owned)
            .collect();

        Ok(Self {
            issuer_url: required("DASH_OIDC_ISSUER_URL")?,
            client_id: required("DASH_OIDC_CLIENT_ID")?,
            client_secret: optional("DASH_OIDC_CLIENT_SECRET"),
            redirect_url: required("DASH_OIDC_REDIRECT_URL")?,
            requested_scopes,
            groups_claim: optional("DASH_OIDC_GROUPS_CLAIM").unwrap_or_else(|| "groups".into()),
            scopes_claim: optional("DASH_OIDC_SCOPES_CLAIM").unwrap_or_else(|| "scope".into()),
            administrator_group: optional("DASH_OIDC_ADMIN_GROUP"),
            post_logout_redirect_url: optional("DASH_OIDC_POST_LOGOUT_REDIRECT_URL"),
        })
    }
}

/// Verified profile and authorization claims returned by the provider.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OidcIdentity {
    pub issuer: String,
    pub subject: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub groups: Vec<String>,
    pub scopes: Vec<String>,
    pub is_administrator: bool,
}

#[derive(Debug, Error)]
pub enum OidcProviderError {
    #[error("invalid OIDC configuration: {0}")]
    Configuration(String),
    #[error("OIDC provider discovery failed: {0}")]
    Discovery(String),
    #[error("OIDC login transaction is missing or expired")]
    InvalidState,
    #[error("OIDC callback is missing the authorization code")]
    MissingCode,
    #[error("OIDC callback failed: {0}")]
    Callback(String),
    #[error("OIDC user information request failed: {0}")]
    UserInfo(String),
    #[error("OIDC identity has no usable username")]
    MissingUsername,
}

struct LoginTransaction {
    csrf: CsrfToken,
    nonce: Nonce,
    pkce_verifier: PkceCodeVerifier,
    created_at: Instant,
}

/// Async OpenID Connect authorization-code flow with state, nonce, and PKCE checks.
///
/// Pending transactions live in process memory. Deploy one application instance,
/// or replace this in-memory store with a shared expiring store before scaling out.
pub struct OidcProvider {
    config: OidcConfig,
    metadata: ProviderMetadataWithLogout,
    http: openidconnect::reqwest::Client,
    pending: Mutex<HashMap<String, LoginTransaction>>,
}

impl OidcProvider {
    pub async fn discover(config: OidcConfig) -> Result<Self, OidcProviderError> {
        let issuer = IssuerUrl::new(config.issuer_url.clone())
            .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;
        RedirectUrl::new(config.redirect_url.clone())
            .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;

        let http = openidconnect::reqwest::ClientBuilder::new()
            // Never follow redirects while sending credentials to provider endpoints.
            .redirect(openidconnect::reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;
        let metadata = ProviderMetadataWithLogout::discover_async(issuer, &http)
            .await
            .map_err(|error| OidcProviderError::Discovery(error.to_string()))?;
        if metadata.token_endpoint().is_none() || metadata.userinfo_endpoint().is_none() {
            return Err(OidcProviderError::Configuration(
                "the discovered provider must expose token and user-info endpoints".to_owned(),
            ));
        }

        Ok(Self {
            config,
            metadata,
            http,
            pending: Mutex::new(HashMap::new()),
        })
    }

    /// Starts login and returns the provider authorization URL.
    pub fn begin_login(&self) -> Result<String, OidcProviderError> {
        let client = self.client()?;
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
        let mut request = client.authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );
        for scope in &self.config.requested_scopes {
            request = request.add_scope(Scope::new(scope.clone()));
        }
        let (url, csrf, nonce) = request.set_pkce_challenge(challenge).url();

        let mut pending = self
            .pending
            .lock()
            .map_err(|_| OidcProviderError::InvalidState)?;
        pending.retain(|_, flow| flow.created_at.elapsed().as_secs() < 600);
        pending.insert(
            csrf.secret().to_owned(),
            LoginTransaction {
                csrf,
                nonce,
                pkce_verifier: verifier,
                created_at: Instant::now(),
            },
        );
        Ok(url.to_string())
    }

    /// Exchanges a callback code, verifies the ID token, and loads provider claims.
    pub async fn complete_login(
        &self,
        code: &str,
        state: &str,
    ) -> Result<OidcIdentity, OidcProviderError> {
        if code.trim().is_empty() {
            return Err(OidcProviderError::MissingCode);
        }
        let transaction = self
            .pending
            .lock()
            .map_err(|_| OidcProviderError::InvalidState)?
            .remove(state)
            .filter(|flow| flow.created_at.elapsed().as_secs() < 600)
            .ok_or(OidcProviderError::InvalidState)?;
        if transaction.csrf.secret() != state {
            return Err(OidcProviderError::InvalidState);
        }

        let client = self.client()?;
        let token_response = client
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .map_err(|error| OidcProviderError::Callback(error.to_string()))?
            .set_pkce_verifier(transaction.pkce_verifier)
            .request_async(&self.http)
            .await
            .map_err(|error| OidcProviderError::Callback(error.to_string()))?;
        let id_token = token_response
            .id_token()
            .ok_or_else(|| OidcProviderError::Callback("provider omitted ID token".to_owned()))?;
        let verifier = client.id_token_verifier();
        let claims = id_token
            .claims(&verifier, &transaction.nonce)
            .map_err(|error| OidcProviderError::Callback(error.to_string()))?;

        if let Some(expected_hash) = claims.access_token_hash() {
            let actual_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                id_token
                    .signing_alg()
                    .map_err(|error| OidcProviderError::Callback(error.to_string()))?,
                id_token
                    .signing_key(&verifier)
                    .map_err(|error| OidcProviderError::Callback(error.to_string()))?,
            )
            .map_err(|error| OidcProviderError::Callback(error.to_string()))?;
            if actual_hash != *expected_hash {
                return Err(OidcProviderError::Callback(
                    "access-token hash mismatch".to_owned(),
                ));
            }
        }

        let mut identity = self
            .load_user_info(token_response.access_token().secret())
            .await?;
        if identity.subject != claims.subject().as_str() {
            return Err(OidcProviderError::Callback(
                "user-info subject did not match ID token".to_owned(),
            ));
        }
        identity.issuer = self.config.issuer_url.clone();
        if let Some(scopes) = token_response.scopes() {
            identity.scopes = scopes
                .iter()
                .map(|scope| scope.as_str().to_owned())
                .collect();
        }
        self.set_administrator(&mut identity);
        Ok(identity)
    }

    /// Validates a bearer token by presenting it to the provider's user-info endpoint.
    pub async fn validate_bearer(
        &self,
        access_token: &str,
    ) -> Result<OidcIdentity, OidcProviderError> {
        let mut identity = self.load_user_info(access_token).await?;
        identity.issuer = self.config.issuer_url.clone();
        self.set_administrator(&mut identity);
        Ok(identity)
    }

    /// Creates an RP-initiated logout URL when the provider advertises support.
    pub fn end_session_url(&self) -> Result<Option<String>, OidcProviderError> {
        let Some(endpoint) = self
            .metadata
            .additional_metadata()
            .end_session_endpoint
            .clone()
        else {
            return Ok(None);
        };
        let redirect = match &self.config.post_logout_redirect_url {
            Some(value) => value.clone(),
            None => {
                let mut redirect = url::Url::parse(&self.config.redirect_url)
                    .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;
                redirect.set_path("/");
                redirect.set_query(None);
                redirect.set_fragment(None);
                redirect.to_string()
            }
        };
        let redirect = PostLogoutRedirectUrl::new(redirect)
            .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;
        let url = LogoutRequest::from(endpoint)
            .set_client_id(ClientId::new(self.config.client_id.clone()))
            .set_post_logout_redirect_uri(redirect)
            .http_get_url();
        Ok(Some(url.to_string()))
    }

    fn client(&self) -> Result<DiscoveredClient, OidcProviderError> {
        let redirect = RedirectUrl::new(self.config.redirect_url.clone())
            .map_err(|error| OidcProviderError::Configuration(error.to_string()))?;
        let secret = self.config.client_secret.clone().map(ClientSecret::new);
        Ok(CoreClient::from_provider_metadata(
            self.metadata.clone(),
            ClientId::new(self.config.client_id.clone()),
            secret,
        )
        .set_redirect_uri(redirect))
    }

    fn set_administrator(&self, identity: &mut OidcIdentity) {
        identity.is_administrator = self
            .config
            .administrator_group
            .as_ref()
            .is_some_and(|required| identity.groups.iter().any(|group| group == required));
    }

    async fn load_user_info(&self, access_token: &str) -> Result<OidcIdentity, OidcProviderError> {
        let endpoint = self
            .metadata
            .userinfo_endpoint()
            .ok_or_else(|| OidcProviderError::Configuration("missing user-info endpoint".into()))?;
        let response = self
            .http
            .get(endpoint.url().clone())
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|error| OidcProviderError::UserInfo(error.to_string()))?
            .error_for_status()
            .map_err(|error| OidcProviderError::UserInfo(error.to_string()))?;
        let body = response
            .text()
            .await
            .map_err(|error| OidcProviderError::UserInfo(error.to_string()))?;
        let claims: UserInfo = serde_json::from_str(&body)
            .map_err(|error| OidcProviderError::UserInfo(error.to_string()))?;
        let username = claims
            .preferred_username
            .or_else(|| claims.name.clone())
            .filter(|name| !name.trim().is_empty())
            .ok_or(OidcProviderError::MissingUsername)?;
        let groups = claims
            .extra
            .get(&self.config.groups_claim)
            .and_then(value_list)
            .unwrap_or_default();
        let scopes = claims
            .extra
            .get(&self.config.scopes_claim)
            .and_then(value_list)
            .unwrap_or_default();
        Ok(OidcIdentity {
            issuer: String::new(),
            subject: claims.sub,
            username,
            first_name: claims.given_name,
            last_name: claims.family_name,
            profile_picture: claims.picture,
            groups,
            scopes,
            is_administrator: false,
        })
    }
}

#[derive(Deserialize)]
struct UserInfo {
    sub: String,
    #[serde(default)]
    preferred_username: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    given_name: Option<String>,
    #[serde(default)]
    family_name: Option<String>,
    #[serde(default)]
    picture: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

fn value_list(value: &serde_json::Value) -> Option<Vec<String>> {
    match value {
        serde_json::Value::String(value) => {
            Some(value.split_whitespace().map(str::to_owned).collect())
        }
        serde_json::Value::Array(values) => Some(
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_owned)
                .collect(),
        ),
        _ => None,
    }
}

#[cfg(test)]
#[path = "oidc_test.rs"]
mod oidc_test;
