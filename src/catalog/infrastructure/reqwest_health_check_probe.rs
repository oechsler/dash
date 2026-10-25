use std::time::Duration;

use crate::catalog::application::ports::HealthCheckProbe;

#[derive(Clone)]
pub struct ReqwestHealthCheckProbe {
    verified_client: reqwest::Client,
    insecure_client: reqwest::Client,
}

impl ReqwestHealthCheckProbe {
    pub fn new() -> Result<Self, reqwest::Error> {
        let verified_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let insecure_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            verified_client,
            insecure_client,
        })
    }
}

#[async_trait::async_trait]
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
