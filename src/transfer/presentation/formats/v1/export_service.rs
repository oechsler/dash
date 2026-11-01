use crate::{
    shared::application::ApplicationError,
    shared::domain::UserId,
    transfer::application::{TransferError, models::UserDataSnapshot},
};

pub(super) struct ExportService;

impl ExportService {
    pub(super) fn export(
        &self,
        _snapshot: UserDataSnapshot,
        _owner: &UserId,
        _username: String,
        _include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError> {
        Err(TransferError::UnsupportedOperation {
            version: 1,
            operation: crate::transfer::application::TransferOperation::Export,
        }
        .into())
    }
}
