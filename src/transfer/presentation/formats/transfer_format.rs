use crate::{
    shared::application::ApplicationError, shared::domain::UserId,
    transfer::application::models::UserDataSnapshot,
};

pub trait TransferFormat: Send + Sync {
    fn version(&self) -> u32;

    fn import(&self, payload: &[u8], owner: &UserId) -> Result<UserDataSnapshot, ApplicationError>;

    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> Result<Vec<u8>, ApplicationError>;
}
