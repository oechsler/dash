use crate::{
    shared::application::ApplicationError,
    transfer::application::{models::formats::v2::UserDataExport, ports::UserDataReader},
    user::domain::UserId,
};

use crate::transfer::application::{models::formats::ExportFormat, services::ExportService};

pub struct ExportUserDataInput {
    pub owner: UserId,
    pub username: String,
    pub include_applications: bool,
}

pub struct ExportUserData<R, F> {
    repository: R,
    service: ExportService<F>,
}

impl<R: UserDataReader, F: ExportFormat> ExportUserData<R, F> {
    pub fn new(repository: R, service: ExportService<F>) -> Self {
        Self {
            repository,
            service,
        }
    }

    pub async fn execute(
        &self,
        input: ExportUserDataInput,
    ) -> Result<UserDataExport, ApplicationError> {
        let snapshot = self.repository.load(&input.owner).await?;
        self.service.export(
            snapshot,
            &input.owner,
            input.username,
            input.include_applications,
        )
    }
}
