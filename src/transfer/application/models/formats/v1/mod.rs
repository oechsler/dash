mod export;
mod import;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

use super::TransferFormat;

pub struct V1Format;

impl TransferFormat for V1Format {
    fn version(&self) -> u32 {
        1
    }

    fn import(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        import::import(payload, owner)
    }

    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError> {
        export::export(snapshot, owner, username, include_applications)
    }
}
