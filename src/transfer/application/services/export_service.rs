use crate::{
    shared::application::ApplicationError,
    transfer::application::models::{UserDataSnapshot, formats::v2::UserDataExport},
    user::domain::UserId,
};

use crate::transfer::application::models::formats::ExportFormat;

pub struct ExportService<F> {
    format: F,
}

impl<F: ExportFormat> ExportService<F> {
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
    ) -> Result<UserDataExport, ApplicationError> {
        self.format
            .export(snapshot, owner, username, include_applications)
    }
}
