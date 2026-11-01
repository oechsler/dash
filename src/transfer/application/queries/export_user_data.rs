use crate::{
    shared::application::ApplicationError,
    shared::domain::UserId,
    transfer::application::{models::UserDataSnapshot, ports::UserDataSnapshotReader},
};

pub struct ExportUserDataInput {
    pub owner: UserId,
}

pub struct ExportUserData<R> {
    repository: R,
}

impl<R: UserDataSnapshotReader> ExportUserData<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: ExportUserDataInput,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        self.repository
            .load(&input.owner)
            .await
            .map_err(ApplicationError::from)
    }
}
