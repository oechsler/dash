use crate::{
    shared::application::ApplicationError,
    shared::domain::UserId,
    transfer::application::{
        models::UserDataSnapshot,
        ports::{UserDataSnapshotReader, UserDataSnapshotWriter},
        services::UserDataMerger,
    },
};

pub struct ImportUserDataInput {
    pub owner: UserId,
    pub is_admin: bool,
    pub snapshot: UserDataSnapshot,
}

pub struct ImportUserData<R, W> {
    reader: R,
    writer: W,
    user_data_merger: UserDataMerger,
}

impl<R: UserDataSnapshotReader, W: UserDataSnapshotWriter> ImportUserData<R, W> {
    pub fn new(reader: R, writer: W, user_data_merger: UserDataMerger) -> Self {
        Self {
            reader,
            writer,
            user_data_merger,
        }
    }

    pub async fn execute(&self, input: ImportUserDataInput) -> Result<(), ApplicationError> {
        let mut snapshot = self.reader.load(&input.owner).await?;
        self.user_data_merger.merge(
            &mut snapshot,
            &input.snapshot,
            &input.owner,
            input.is_admin,
        )?;
        self.writer
            .save(&input.owner, &snapshot)
            .await
            .map_err(ApplicationError::from)
    }
}
