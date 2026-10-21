#[cfg(test)]
mod tests;

use crate::{
    shared::application::ApplicationError,
    transfer::application::{error::TransferError, models::UserDataSnapshot},
    user::domain::UserId,
};

pub(super) fn export(
    _snapshot: UserDataSnapshot,
    _owner: &UserId,
    _username: String,
    _include_applications: bool,
) -> Result<Vec<u8>, ApplicationError> {
    Err(TransferError::UnsupportedOperation {
        version: 1,
        operation: crate::transfer::application::error::TransferOperation::Export,
    }
    .into())
}
