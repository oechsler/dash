use crate::{
    catalog::domain::{Application, ApplicationHealthCheck, ApplicationId, ApplicationVisibility},
    dashboard::domain::WebAddress,
    icon::domain::IconReference,
    shared::application::ApplicationError,
    user::domain::GroupId,
};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

pub struct CreateApplicationInput {
    pub name: String,
    pub icon: String,
    pub address: String,
    pub visible_to_groups: Vec<String>,
    pub health_check: Option<ApplicationHealthCheck>,
}

pub struct CreateApplication<R> {
    repository: R,
}

impl<R: ApplicationReader + ApplicationWriter> CreateApplication<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateApplicationInput,
    ) -> Result<ApplicationId, ApplicationError> {
        let groups = parse_groups(input.visible_to_groups)?;
        let mut application = Application::create(
            input.name,
            IconReference::parse(input.icon)?,
            WebAddress::parse(input.address)?,
            ApplicationVisibility::groups(groups),
        )?;
        application.set_health_check(input.health_check);
        let id = application.id();
        self.repository.save(&application).await?;
        Ok(id)
    }
}

fn parse_groups(groups: Vec<String>) -> Result<Vec<GroupId>, ApplicationError> {
    groups
        .into_iter()
        .map(GroupId::new)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}
