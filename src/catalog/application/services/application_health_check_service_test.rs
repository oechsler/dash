use crate::shared::application::PortResult;
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
    icon::domain::IconReference,
    shared::domain::WebAddress,
};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

#[derive(Clone, Default)]
struct MemoryApplications {
    applications: Arc<Mutex<Vec<Application>>>,
}

#[async_trait]
impl ApplicationReader for MemoryApplications {
    async fn list(&self) -> PortResult<Vec<Application>> {
        Ok(self.applications.lock().unwrap().clone())
    }

    async fn get(&self, id: ApplicationId) -> PortResult<Option<Application>> {
        Ok(self
            .applications
            .lock()
            .unwrap()
            .iter()
            .find(|application| application.id() == id)
            .cloned())
    }
}

#[async_trait]
impl ApplicationWriter for MemoryApplications {
    async fn save(&self, application: &Application) -> PortResult<()> {
        let mut applications = self.applications.lock().unwrap();
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

    async fn delete(&self, id: ApplicationId) -> PortResult<()> {
        self.applications
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
        IconReference::try_from("mdi:portal").unwrap(),
        WebAddress::try_from("https://portal.example").unwrap(),
        ApplicationVisibility::everyone(),
    )
    .unwrap();
    application.set_health_check(Some(ApplicationHealthCheck::new(
        Some(HealthCheckUrl::try_from("https://portal.example/health").unwrap()),
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
        IconReference::try_from("mdi:portal").unwrap(),
        WebAddress::try_from("https://portal.example").unwrap(),
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
