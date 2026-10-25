use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;

use crate::{
    catalog::{
        application::{ports::HealthCheckProbe, services::ApplicationHealthCheckService},
        domain::{
            Application, ApplicationHealth, ApplicationHealthCheck, ApplicationId,
            ApplicationVisibility, ExpectedStatusClass, HealthCheckUrl,
        },
    },
    dashboard::domain::WebAddress,
    icon::domain::IconReference,
    shared::application::ApplicationError,
};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

#[derive(Clone, Default)]
struct MemoryApplications(Arc<Mutex<Vec<Application>>>);

#[async_trait]
impl ApplicationReader for MemoryApplications {
    async fn list(&self) -> Result<Vec<Application>, ApplicationError> {
        Ok(self.0.lock().unwrap().clone())
    }

    async fn get(&self, id: ApplicationId) -> Result<Option<Application>, ApplicationError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .find(|application| application.id() == id)
            .cloned())
    }
}

#[async_trait]
impl ApplicationWriter for MemoryApplications {
    async fn save(&self, application: &Application) -> Result<(), ApplicationError> {
        let mut applications = self.0.lock().unwrap();
        if let Some(existing) = applications
            .iter_mut()
            .find(|existing| existing.id() == application.id())
        {
            *existing = application.clone();
        } else {
            applications.push(application.clone());
        }
        Ok(())
    }

    async fn delete(&self, id: ApplicationId) -> Result<(), ApplicationError> {
        self.0
            .lock()
            .unwrap()
            .retain(|application| application.id() != id);
        Ok(())
    }
}

#[derive(Clone)]
struct FixedProbe {
    status: Option<u16>,
    calls: Arc<Mutex<Vec<(String, bool, Duration)>>>,
}

#[async_trait]
impl HealthCheckProbe for FixedProbe {
    async fn status_code(
        &self,
        url: &str,
        ignore_tls_errors: bool,
        timeout: Duration,
    ) -> Option<u16> {
        self.calls
            .lock()
            .unwrap()
            .push((url.to_owned(), ignore_tls_errors, timeout));
        self.status
    }
}

#[tokio::test]
async fn check_all_uses_the_override_and_configured_status_and_tls_options() {
    let mut application = Application::create(
        "Portal",
        IconReference::parse("mdi:portal").unwrap(),
        WebAddress::parse("https://portal.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    application.set_health_check(Some(ApplicationHealthCheck::new(
        Some(HealthCheckUrl::parse("https://portal.example/health").unwrap()),
        ExpectedStatusClass::ClientError4xx,
        true,
    )));
    let repository = MemoryApplications::default();
    repository.save(&application).await.unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let service = ApplicationHealthCheckService::new(
        repository.clone(),
        FixedProbe {
            status: Some(401),
            calls: calls.clone(),
        },
    );

    service.check_all().await.unwrap();

    let checked = repository.get(application.id()).await.unwrap().unwrap();
    assert_eq!(checked.health(), Some(ApplicationHealth::Up));
    assert_eq!(
        calls.lock().unwrap().as_slice(),
        &[(
            "https://portal.example/health".to_owned(),
            true,
            Duration::from_secs(5),
        )]
    );
}

#[tokio::test]
async fn check_all_does_not_probe_apps_without_health_checks() {
    let application = Application::create(
        "Portal",
        IconReference::parse("mdi:portal").unwrap(),
        WebAddress::parse("https://portal.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    let repository = MemoryApplications::default();
    repository.save(&application).await.unwrap();
    let calls = Arc::new(Mutex::new(Vec::new()));
    let service = ApplicationHealthCheckService::new(
        repository,
        FixedProbe {
            status: Some(200),
            calls: calls.clone(),
        },
    );

    service.check_all().await.unwrap();

    assert!(calls.lock().unwrap().is_empty());
}
