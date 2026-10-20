mod decode;

use crate::{
    shared::application::ApplicationError,
    transfer::application::{error::TransferError, models::UserDataSnapshot},
    user::domain::UserId,
};

use super::model::UserDataExport;

pub(super) fn decode(payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError> {
    let document: UserDataExport = serde_json::from_slice(payload)?;
    if document.signature.is_none() {
        return Err(TransferError::MissingSignature.into());
    }
    if !document.verify()? {
        return Err(TransferError::InvalidSignature.into());
    }
    decode::into_snapshot(owner, document)
}

pub(super) fn decode_export(
    owner: &UserId,
    document: UserDataExport,
) -> Result<UserDataSnapshot, ApplicationError> {
    decode::into_snapshot(owner, document)
}
