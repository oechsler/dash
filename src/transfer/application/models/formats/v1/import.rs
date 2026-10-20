use crate::{
    shared::application::ApplicationError, transfer::application::models::UserDataSnapshot,
    user::domain::UserId,
};

mod mapping;
mod signature;

pub(super) fn decode(payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
    let document = signature::parse(payload)?.into_v2_export();
    super::super::v2::V2Format::decode_document(owner, document)
}
