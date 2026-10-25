use crate::{
    shared::application::ApplicationError,
    transfer::application::{
        models::UserDataSnapshot,
        ports::{UserDataSnapshotReader, UserDataSnapshotWriter},
        services::MergeUserDataService,
    },
    user::domain::UserId,
};

pub struct ImportUserDataInput {
    pub owner: UserId,
    pub is_admin: bool,
    pub snapshot: UserDataSnapshot,
}

pub struct ImportUserData<R, W> {
    reader: R,
    writer: W,
    merge_user_data: MergeUserDataService,
}

impl<R: UserDataSnapshotReader, W: UserDataSnapshotWriter> ImportUserData<R, W> {
    pub fn new(reader: R, writer: W, merge_user_data: MergeUserDataService) -> Self {
        Self {
            reader,
            writer,
            merge_user_data,
        }
    }

    pub async fn execute(&self, input: ImportUserDataInput) -> Result<(), ApplicationError> {
        let mut snapshot = self.reader.load(&input.owner).await?;
        self.merge_user_data
            .merge(&mut snapshot, &input.snapshot, &input.owner, input.is_admin)?;
        self.writer.save(&input.owner, &snapshot).await
    }
}
