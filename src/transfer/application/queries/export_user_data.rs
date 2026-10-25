use crate::{
    shared::application::ApplicationError,
    transfer::application::{models::UserDataSnapshot, ports::UserDataSnapshotReader},
    user::domain::UserId,
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
        self.repository.load(&input.owner).await
    }
}
