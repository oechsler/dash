use crate::{
    shared::application::ApplicationError,
    transfer::application::models::{UserDataSnapshot, formats::TransferFormat},
    user::domain::UserId,
};

pub struct ExportService<F> {
    format: F,
}

impl<F: TransferFormat> ExportService<F> {
    pub fn new(format: F) -> Self {
        Self { format }
    }

    pub fn version(&self) -> u32 {
        self.format.version()
    }

    pub fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError> {
        self.format
            .export(snapshot, owner, username, include_applications)
    }
}
