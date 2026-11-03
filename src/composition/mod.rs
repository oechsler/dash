//! Composition root for application-wide infrastructure.

pub mod migrations;
#[cfg(feature = "postgres")]
mod postgres_connector;

#[cfg(feature = "postgres")]
pub use postgres_connector::{PostgresConnector, PostgresConnectorResult};

#[cfg(feature = "postgres")]
impl PostgresConnector {
    /// Connects, applies migrations, and builds the HTTP API composition.
    pub async fn http_api_from_env()
    -> Result<crate::presentation::http::HttpApi, Box<dyn std::error::Error + Send + Sync>> {
        let signing_key = std::env::var("DASH_SESSION_SIGNING_KEY")?;
        if signing_key.len() < 32 {
            return Err("DASH_SESSION_SIGNING_KEY must contain at least 32 bytes".into());
        }
        let database_url = std::env::var("DATABASE_URL")?;
        let connector = Self::connect(&database_url).await?;
        connector.migrate().await?;
        let oidc_config = crate::presentation::http::auth::OidcConfig::from_env()?;
        let oidc = crate::presentation::http::auth::OidcProvider::discover(oidc_config).await?;
        Ok(crate::presentation::http::HttpApi::new(
            connector.connection_handle(),
            std::sync::Arc::new(oidc),
            signing_key.into_bytes(),
        )?)
    }
}
