use crate::shared::application::ApplicationResult;
use crate::{shared::domain::UserId, transfer::application::models::UserDataSnapshot};

pub trait TransferFormat: Send + Sync {
    fn version(&self) -> u32;

    fn import(&self, payload: &[u8], owner: &UserId) -> ApplicationResult<UserDataSnapshot>;

    fn export(
        &self,
        snapshot: UserDataSnapshot,
        owner: &UserId,
        username: String,
        include_applications: bool,
    ) -> ApplicationResult<Vec<u8>>;
}
