use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

use super::super::TransferFormat;

pub struct V1Format;

impl TransferFormat for V1Format {
    fn version(&self) -> u32 {
        1
    }

    fn import(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        super::import_service::ImportService.import(payload, owner)
    }

    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError> {
        super::export_service::ExportService.export(snapshot, owner, username, include_applications)
    }
}
