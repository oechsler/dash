use crate::shared::application::ApplicationResult;
use crate::{
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
    ) -> ApplicationResult<Vec<u8>> {
        Err(TransferError::UnsupportedOperation {
            version: 1,
            operation: crate::transfer::application::TransferOperation::Export,
        }
        .into())
    }
}
