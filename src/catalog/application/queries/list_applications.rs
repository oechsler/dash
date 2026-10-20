use crate::{catalog::domain::Application, shared::application::ApplicationError};

use crate::catalog::application::ports::ApplicationReader;

pub struct ListApplications<R> {
    repository: R,
}

impl<R: ApplicationReader> ListApplications<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<Application>, ApplicationError> {
        self.repository.list().await
    }
}
