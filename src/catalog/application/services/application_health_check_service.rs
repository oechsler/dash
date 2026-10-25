use std::time::Duration;

use crate::{
    catalog::application::ports::{ApplicationReader, ApplicationWriter, HealthCheckProbe},
    shared::application::ApplicationError,
};

const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

/// Checks configured shared apps periodically. The host starts this service as
/// a background task after application repositories and the HTTP probe exist.
pub struct ApplicationHealthCheckService<R, P> {
    applications: R,
    probe: P,
}

impl<R, P> ApplicationHealthCheckService<R, P>
where
    R: ApplicationReader + ApplicationWriter,
    P: HealthCheckProbe,
{
    pub fn new(applications: R, probe: P) -> Self {
        Self {
            applications,
            probe,
        }
    }

    pub async fn check_all(&self) -> Result<(), ApplicationError> {
        let applications = self.applications.list().await?;
        for mut application in applications {
            let Some(target) = application.health_check_target() else {
                continue;
            };
            let ignore_tls_errors = application
                .health_check()
                .is_some_and(|config| config.ignore_tls_errors());
            let status = self
                .probe
                .status_code(target, ignore_tls_errors, HEALTH_CHECK_TIMEOUT)
                .await;
            application.record_health_check(status);
            self.applications.save(&application).await?;
        }
        Ok(())
    }

    pub async fn run_periodically(&self) -> ! {
        let mut interval = tokio::time::interval(HEALTH_CHECK_INTERVAL);
        loop {
            interval.tick().await;
            if let Err(error) = self.check_all().await {
                tracing::error!(%error, "application health-check cycle failed");
            }
        }
    }
}
