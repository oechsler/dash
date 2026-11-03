use crate::{
    shared::{application::ApplicationResult, domain::UserId},
    transfer::application::{
        models::UserDataSnapshot,
        ports::UserDataSnapshotReader,
        queries::{ExportUserData, ExportUserDataInput},
    },
};

/// Handles `GET /api/v1/transfer/export`.
pub async fn handle<R: UserDataSnapshotReader>(
    snapshots: R,
    owner: UserId,
) -> ApplicationResult<UserDataSnapshot> {
    ExportUserData::new(snapshots)
        .execute(ExportUserDataInput { owner })
        .await
}
