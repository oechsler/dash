mod import;
mod model;

use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

use super::ImportFormat;

pub struct V1Format;

impl ImportFormat for V1Format {
    fn version(&self) -> u32 {
        1
    }

    fn decode(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
        import::decode(payload, owner)
    }
}
