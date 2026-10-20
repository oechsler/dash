use crate::{
    catalog::domain::{Application, ApplicationId},
    shared::application::ApplicationError,
};

use crate::catalog::application::ports::ApplicationReader;

pub struct GetApplication<R> {
    repository: R,
}

impl<R: ApplicationReader> GetApplication<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: ApplicationId) -> Result<Application, ApplicationError> {
        self.repository
            .get(id)
            .await?
            .ok_or(ApplicationError::NotFound("application"))
    }
}
