use crate::{shared::application::ApplicationError, transfer::application::error::TransferError};

use super::model::UserDataExport;

pub(super) fn parse(payload: &[u8]) -> Result<UserDataExport, ApplicationError> {
    let document: UserDataExport = serde_json::from_slice(payload)?;
    if document.signature.is_none() {
        return Err(TransferError::MissingSignature.into());
    }
    if !document.verify()? {
        return Err(TransferError::InvalidSignature.into());
    }
    Ok(document)
}
