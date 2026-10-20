use async_trait::async_trait;

use crate::{
    catalog::{
        application::ports::ApplicationReader,
        domain::{Application, ApplicationVisibility},
    },
    dashboard::{
        application::{ports::DashboardReader, queries::GetUserDashboard},
        domain::{UserDashboard, WebAddress},
    },
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::{GroupId, UserId},
};

struct Dashboards {
    result: Option<UserDashboard>,
}

#[async_trait]
impl DashboardReader for Dashboards {
    async fn load(&self, _user_id: &UserId) -> Result<Option<UserDashboard>, ApplicationError> {
        Ok(self.result.clone())
    }
}

struct Applications(Vec<Application>);

#[async_trait]
impl ApplicationReader for Applications {
    async fn list(&self) -> Result<Vec<Application>, ApplicationError> {
        Ok(self.0.clone())
    }

    async fn get(
        &self,
        id: crate::catalog::domain::ApplicationId,
    ) -> Result<Option<Application>, ApplicationError> {
        Ok(self
            .0
            .iter()
            .find(|application| application.id() == id)
            .cloned())
    }
}

fn app(name: &str, visibility: ApplicationVisibility) -> Application {
    Application::builder()
        .name(name)
        .icon(IconReference::parse("mdi:home").unwrap())
        .address(WebAddress::parse("https://example.test").unwrap())
        .visibility(visibility)
        .build()
        .unwrap()
}

#[tokio::test]
async fn query_returns_empty_dashboard_and_filters_applications() {
    let group = GroupId::new("staff").unwrap();
    let dashboards = Dashboards { result: None };
    let query = GetUserDashboard::new(
        dashboards,
        Applications(vec![
            app("Public", ApplicationVisibility::everyone()),
            app("Staff", ApplicationVisibility::groups([group.clone()])),
            app(
                "Admins",
                ApplicationVisibility::groups([GroupId::new("admins").unwrap()]),
            ),
        ]),
    );

    let view = query
        .execute(
            &UserId::new("user-1").unwrap(),
            &std::collections::HashSet::from([group]),
        )
        .await
        .unwrap();

    assert!(view.dashboard.categories().is_empty());
    assert_eq!(view.applications.len(), 2);
    assert_eq!(view.applications[0].name(), "Public");
    assert_eq!(view.applications[1].name(), "Staff");
}
