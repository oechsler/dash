use crate::presentation::http::auth::{OidcProvider, OidcProviderError};

/// Handles `GET /api/v1/sessions/login`.
pub fn handle(provider: &OidcProvider) -> Result<String, OidcProviderError> {
    provider.begin_login()
}
