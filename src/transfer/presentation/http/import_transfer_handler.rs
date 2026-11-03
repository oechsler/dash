use crate::{
    shared::{application::ApplicationResult, domain::UserId},
    transfer::{
        application::{
            commands::{ImportUserData, ImportUserDataInput},
            models::UserDataSnapshot,
            ports::{UserDataSnapshotReader, UserDataSnapshotWriter},
            services::UserDataMerger,
        },
        presentation::formats::ImportFormatRegistry,
    },
};

/// Handles `POST /api/v1/transfer/import` after multipart extraction.
pub async fn handle<R, W>(
    reader: R,
    writer: W,
    owner: UserId,
    is_admin: bool,
    payload: &[u8],
) -> ApplicationResult<()>
where
    R: UserDataSnapshotReader,
    W: UserDataSnapshotWriter,
{
    let snapshot: UserDataSnapshot =
        ImportFormatRegistry::supported_versions().import(payload, &owner)?;
    ImportUserData::new(reader, writer, UserDataMerger)
        .execute(ImportUserDataInput {
            owner,
            is_admin,
            snapshot,
        })
        .await
}
