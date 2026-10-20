use crate::{
    catalog::domain::ApplicationId, dashboard::domain::WebAddress, icon::domain::IconReference,
    shared::application::ApplicationError, user::domain::GroupId,
};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

pub struct UpdateApplicationInput {
    pub id: ApplicationId,
    pub name: String,
    pub icon: String,
    pub address: String,
    pub visible_to_groups: Vec<String>,
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
        application.change_icon(IconReference::parse(input.icon)?);
        application.change_address(WebAddress::parse(input.address)?);
        let groups = input
            .visible_to_groups
            .into_iter()
            .map(GroupId::new)
            .collect::<Result<Vec<_>, _>>()?;
        application.restrict_to(groups);
        self.repository.save(&application).await
    }
}
