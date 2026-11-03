use crate::presentation::http::auth::{OidcIdentity, OidcProvider, OidcProviderError};

/// Handles `GET /api/v1/sessions/login/callback` and returns verified identity data.
pub async fn handle(
    provider: &OidcProvider,
    code: &str,
    state: &str,
) -> Result<OidcIdentity, OidcProviderError> {
    provider.complete_login(code, state).await
}
