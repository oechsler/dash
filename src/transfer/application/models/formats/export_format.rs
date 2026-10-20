use crate::{
    shared::application::ApplicationError,
    transfer::application::models::{UserDataSnapshot, formats::v2::UserDataExport},
    user::domain::UserId,
};

pub trait ExportFormat: Send + Sync {
    fn version(&self) -> u32;
    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<UserDataExport, ApplicationError>;
}
