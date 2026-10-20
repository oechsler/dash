mod import;
mod model;
#[cfg(test)]
mod model_test;
pub(super) mod projection;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

pub use model::{
    ApplicationExport, BookmarkExport, CategoryExport, SettingsExport, ThemeExport, UserDataExport,
};

use super::{ExportFormat, ImportFormat};

pub struct V2Format;

impl V2Format {
    pub(super) fn decode_document(
        owner: &UserId,
        document: UserDataExport,
    ) -> Result<UserDataSnapshot, ApplicationError> {
        import::decode_export(owner, document)
    }
}

impl ImportFormat for V2Format {
    fn version(&self) -> u32 {
        2
    }

    fn decode(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        import::decode(payload, owner)
    }
}

impl ExportFormat for V2Format {
    fn version(&self) -> u32 {
        2
    }

    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<UserDataExport, ApplicationError> {
        projection::snapshot::build(snapshot, owner, username, include_applications)
    }
}
