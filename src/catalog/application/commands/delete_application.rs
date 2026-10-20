use crate::{catalog::domain::ApplicationId, shared::application::ApplicationError};

use crate::catalog::application::ports::{ApplicationReader, ApplicationWriter};

pub struct DeleteApplication<R> {
    repository: R,
}

impl<R: ApplicationReader + ApplicationWriter> DeleteApplication<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: ApplicationId) -> Result<(), ApplicationError> {
        self.repository
            .get(id)
            .await?
            .ok_or(ApplicationError::NotFound("application"))?;
        self.repository.delete(id).await
    }
}
