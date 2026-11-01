use async_trait::async_trait;
use std::time::Duration;

use crate::catalog::application::ports::HealthCheckProbe;

/// Result for constructing the HTTP health-check probe.
pub type HealthCheckProbeResult<T> = Result<T, reqwest::Error>;

#[derive(Clone)]
pub struct ReqwestHealthCheckProbe {
    verified_client: reqwest::Client,
    insecure_client: reqwest::Client,
}

impl ReqwestHealthCheckProbe {
    pub fn new() -> HealthCheckProbeResult<Self> {
        Ok(Self {
            verified_client: Self::build_client(false)?,
            insecure_client: Self::build_client(true)?,
        })
    }

    fn build_client(ignore_tls_errors: bool) -> HealthCheckProbeResult<reqwest::Client> {
        let builder = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());
        let builder = if ignore_tls_errors {
            builder.danger_accept_invalid_certs(true)
        } else {
            builder
        };
        builder.build()
    }
}

#[async_trait]
impl HealthCheckProbe for ReqwestHealthCheckProbe {
    async fn status_code(
        &self,
        url: &str,
        ignore_tls_errors: bool,
        timeout: Duration,
    ) -> Option<u16> {
        let client = if ignore_tls_errors {
            &self.insecure_client
        } else {
            &self.verified_client
        };
        client
            .get(url)
            .timeout(timeout)
            .send()
            .await
            .ok()
            .map(|response| response.status().as_u16())
    }
}
