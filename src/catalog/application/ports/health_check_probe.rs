use std::time::Duration;

use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckProbe: Send + Sync {
    /// Returns the HTTP status code, or `None` when the endpoint could not be reached.
    async fn status_code(
        &self,
        url: &str,
        ignore_tls_errors: bool,
        timeout: Duration,
    ) -> Option<u16>;
}
