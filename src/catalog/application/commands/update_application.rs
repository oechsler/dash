use crate::{
    catalog::domain::{ApplicationHealthCheck, ApplicationId},
    icon::domain::IconReference,
    shared::application::ApplicationError,
    shared::domain::GroupId,
    shared::domain::WebAddress,
};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

pub struct UpdateApplicationInput {
    pub id: ApplicationId,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub visible_to_groups: Vec<String>,
    pub health_check: Option<ApplicationHealthCheck>,
}

pub struct UpdateApplication<R> {
    repository: R,
}

impl<R: ApplicationReader + ApplicationWriter> UpdateApplication<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, input: UpdateApplicationInput) -> Result<(), ApplicationError> {
        let mut application = self
            .repository
            .get(input.id)
            .await?
            .ok_or(ApplicationError::NotFound("application"))?;
        application.rename(input.name)?;
        application.change_icon(IconReference::try_from(input.icon)?);
        application.change_address(WebAddress::try_from(input.address)?);
        let groups = input
            .visible_to_groups
            .into_iter()
            .map(GroupId::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        application.restrict_to(groups);
        application.set_health_check(input.health_check);
        self.repository
            .save(&application)
            .await
            .map_err(ApplicationError::from)
    }
}
