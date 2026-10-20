use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{
    dashboard::{
        application::{
            commands::{CreateCategory, CreateCategoryInput},
            ports::{DashboardReader, DashboardWriter},
        },
        domain::{CategoryError, DashboardError, UserDashboard},
    },
    shared::{application::ApplicationError, domain::DomainError},
    user::domain::UserId,
};

#[derive(Clone, Default)]
struct InMemoryDashboards(Arc<Mutex<Option<UserDashboard>>>);

#[async_trait]
impl DashboardReader for InMemoryDashboards {
    async fn load(&self, _user_id: &UserId) -> Result<Option<UserDashboard>, ApplicationError> {
        Ok(self.0.lock().unwrap().clone())
    }
}

#[async_trait]
impl DashboardWriter for InMemoryDashboards {
    async fn save(&self, dashboard: &UserDashboard) -> Result<(), ApplicationError> {
        *self.0.lock().unwrap() = Some(dashboard.clone());
        Ok(())
    }
}

#[tokio::test]
async fn create_category_persists_it_in_the_owners_dashboard() {
    let repository = InMemoryDashboards::default();
    let command = CreateCategory::new(repository.clone());
    let owner = UserId::new("user-1").unwrap();

    let category_id = command
        .execute(CreateCategoryInput {
            owner: owner.clone(),
            name: "  Homelab  ".into(),
            shelved: true,
        })
        .await
        .unwrap();

    let dashboard = repository.load(&owner).await.unwrap().unwrap();
    assert_eq!(dashboard.owner(), &owner);
    assert_eq!(dashboard.categories()[0].id(), category_id);
    assert_eq!(dashboard.categories()[0].name(), "Homelab");
    assert!(dashboard.categories()[0].is_shelved());
}

#[tokio::test]
async fn create_category_keeps_domain_error_context_in_application_error() {
    let repository = InMemoryDashboards::default();
    let command = CreateCategory::new(repository.clone());

    let result = command
        .execute(CreateCategoryInput {
            owner: UserId::new("user-1").unwrap(),
            name: "   ".into(),
            shelved: false,
        })
        .await;

    assert!(matches!(
        result,
        Err(ApplicationError::Domain(DomainError::Dashboard(
            DashboardError::Category(CategoryError::EmptyName { .. })
        )))
    ));
    assert!(repository.0.lock().unwrap().is_none());
}
